#![allow(dead_code)]

use std::{
    collections::HashMap,
    path::PathBuf,
    sync::{Arc, Mutex},
};

use axum_test::TestServer;
use backend::{
    AdminAuth, Keys, app,
    services::{
        container::ContainerServiceTrait,
        project::{ProjectInfo, ProjectServiceError, ProjectServiceTrait},
    },
};
use cookie::Cookie;
use itertools::Itertools;
use serde_json::json;

struct Project {
    name: String,
    dir: PathBuf,
    files: Vec<File>,
}

struct File {
    name: String,
    content: String,
}

pub struct MockProjectService {
    data: Arc<Mutex<Vec<Project>>>,
}

impl Default for MockProjectService {
    fn default() -> Self {
        let data = vec![
            Project {
                name: "test".to_string(),
                dir: "/path/to/project1".into(),
                files: vec![File {
                    name: "compose.yml".to_string(),
                    content: "compose.yml".to_string(),
                }],
            },
            Project {
                name: "test2".to_string(),
                dir: "/path/to/project2".into(),
                files: vec![
                    File {
                        name: "compose.yml".to_string(),
                        content: "compose.yml".to_string(),
                    },
                    File {
                        name: ".env".to_string(),
                        content: ".env".to_string(),
                    },
                ],
            },
            Project {
                name: "test3".to_string(),
                dir: "/path/to/project2".into(),
                files: vec![
                    File {
                        name: "compose.yml".to_string(),
                        content: "compose.yml".to_string(),
                    },
                    File {
                        name: ".env".to_string(),
                        content: ".env".to_string(),
                    },
                ],
            },
        ];

        MockProjectService {
            data: Arc::new(Mutex::new(data)),
        }
    }
}

impl ProjectServiceTrait for MockProjectService {
    fn all_projects(
        &self,
    ) -> backend::services::project::Result<Vec<backend::services::project::ProjectInfo>> {
        Ok(self
            .data
            .lock()
            .unwrap()
            .iter()
            .map(|data| ProjectInfo {
                name: data.name.clone(),
                dir: data.dir.clone(),
            })
            .collect())
    }

    fn project(
        &self,
        name: &str,
    ) -> backend::services::project::Result<backend::services::project::ProjectInfo> {
        self.all_projects()?
            .into_iter()
            .find(|project| project.name == name)
            .ok_or_else(|| ProjectServiceError::ProjectNotFound(name.to_string()))
    }

    fn files(&self, project: &ProjectInfo) -> backend::services::project::Result<Vec<String>> {
        let files = self
            .data
            .lock()
            .unwrap()
            .iter()
            .find(|data| data.name == project.name)
            .unwrap()
            .files
            .iter()
            .map(|file| file.name.clone())
            .collect_vec();

        Ok(files)
    }

    fn read_file(
        &self,
        project: &ProjectInfo,
        file_name: &str,
    ) -> backend::services::project::Result<String> {
        let content = self
            .data
            .lock()
            .unwrap()
            .iter()
            .find(|data| data.name == project.name)
            .unwrap()
            .files
            .iter()
            .find(|file| file.name.as_str() == file_name)
            .ok_or(ProjectServiceError::FileNotFound {
                file: file_name.to_string(),
                project: project.name.to_string(),
            })?
            .content
            .clone();

        Ok(content)
    }

    fn update_file(
        &self,
        project: &ProjectInfo,
        file_name: &str,
        content: &str,
    ) -> backend::services::project::Result<String> {
        let mut data = self.data.lock().unwrap();
        let project = data
            .iter_mut()
            .find(|data| data.name == project.name)
            .unwrap();

        let file = project.files.iter_mut().find(|file| file.name == file_name);

        match file {
            Some(file) => {
                file.content = content.to_string();
            }
            None => {
                project.files.push(File {
                    name: file_name.to_string(),
                    content: content.to_string(),
                });
            }
        }

        Ok(content.to_string())
    }
}

pub struct MockContainerService {
    data: Arc<Mutex<HashMap<String, bool>>>,
}

impl Default for MockContainerService {
    fn default() -> Self {
        let mut map = HashMap::new();

        map.insert("test".to_string(), true);
        map.insert("test2".to_string(), false);
        map.insert("test3".to_string(), true);

        MockContainerService {
            data: Arc::new(Mutex::new(map)),
        }
    }
}

impl ContainerServiceTrait for MockContainerService {
    fn are_online(
        &self,
        projects: &[backend::services::project::ProjectInfo],
    ) -> backend::services::container::Result<Vec<bool>> {
        let result: Result<Vec<_>, _> = projects
            .iter()
            .map(|project| self.is_online(project))
            .collect();

        result
    }

    fn is_online(
        &self,
        project: &backend::services::project::ProjectInfo,
    ) -> backend::services::container::Result<bool> {
        Ok(*self.data.lock().unwrap().get(&project.name).unwrap())
    }

    fn stop(&self, project: &ProjectInfo) -> backend::services::container::Result<()> {
        let mut data = self.data.lock().unwrap();
        let state = data.get_mut(&project.name).unwrap();

        if *state {
            *state = false;
        }

        Ok(())
    }

    fn start(&self, project: &ProjectInfo) -> backend::services::container::Result<()> {
        let mut data = self.data.lock().unwrap();
        let state = data.get_mut(&project.name).unwrap();

        if !*state {
            *state = true;
        }

        Ok(())
    }

    fn pull(&self, _project: &ProjectInfo) -> backend::services::container::Result<()> {
        Ok(())
    }
}

pub fn test_server() -> TestServer {
    let project_service = Arc::new(MockProjectService::default());
    let container_service = Arc::new(MockContainerService::default());
    let app = app(
        project_service.clone(),
        container_service.clone(),
        Keys::new("secret".as_bytes()),
        AdminAuth {
            name: "admin".to_string(),
            password: "password".to_string(),
        },
    );

    TestServer::builder().http_transport().build(app).unwrap()
}

pub async fn auth_test_server() -> (TestServer, String) {
    let mut server = test_server();

    let response = server
        .post("/auth")
        .json(&json!({
            "user": "admin",
            "pw": "password",
        }))
        .await;

    let json: serde_json::Value = response.json();
    let token = json.get("token").unwrap().as_str().unwrap().to_string();

    server.add_cookie(Cookie::new("token", token.clone()));

    (server, token)
}
