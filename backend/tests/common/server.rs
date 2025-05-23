#![allow(dead_code)]

use std::{
    collections::HashMap,
    path::PathBuf,
    sync::{Arc, Mutex},
};

use axum_test::TestServer;
use backend::{
    app,
    services::{
        container::{ContainerServiceError, ContainerServiceTrait},
        project::{ProjectInfo, ProjectServiceError, ProjectServiceTrait},
    },
};

struct Project {
    name: String,
    dir: PathBuf,
    compose: String,
    env: Option<String>,
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
                compose: "compose.yml".to_string(),
                env: None,
            },
            Project {
                name: "test2".to_string(),
                dir: "/path/to/project2".into(),
                compose: "compose.yml".to_string(),
                env: Some(".env".to_string()),
            },
            Project {
                name: "test3".to_string(),
                dir: "/path/to/project2".into(),
                compose: "compose.yml".to_string(),
                env: Some(".env".to_string()),
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
            .ok_or_else(|| ProjectServiceError::NotFound(name.to_string()))
    }

    fn compose(
        &self,
        project: &backend::services::project::ProjectInfo,
    ) -> backend::services::project::Result<String> {
        let compose = self
            .data
            .lock()
            .unwrap()
            .iter()
            .find(|data| data.name == project.name)
            .unwrap()
            .compose
            .clone();
        Ok(compose)
    }

    fn env(
        &self,
        project: &backend::services::project::ProjectInfo,
    ) -> backend::services::project::Result<Option<String>> {
        let env = self
            .data
            .lock()
            .unwrap()
            .iter()
            .find(|data| data.name == project.name)
            .unwrap()
            .env
            .clone();
        Ok(env)
    }

    fn update_compose(
        &self,
        project: &ProjectInfo,
        compose: String,
    ) -> backend::services::project::Result<String> {
        let mut data = self.data.lock().unwrap();
        let state = data
            .iter_mut()
            .find(|data| data.name == project.name)
            .unwrap();

        state.compose = compose.clone();
        Ok(compose)
    }

    fn update_env(
        &self,
        project: &ProjectInfo,
        env: String,
    ) -> backend::services::project::Result<String> {
        let mut data = self.data.lock().unwrap();
        let state = data
            .iter_mut()
            .find(|data| data.name == project.name)
            .unwrap();

        state.env = Some(env.clone());
        Ok(env)
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
        } else {
            return Err(ContainerServiceError::AlreadyStopped(project.name.clone()));
        }

        Ok(())
    }

    fn start(&self, project: &ProjectInfo) -> backend::services::container::Result<()> {
        let mut data = self.data.lock().unwrap();
        let state = data.get_mut(&project.name).unwrap();

        if !*state {
            *state = true;
        } else {
            return Err(ContainerServiceError::AlreadyRunning(project.name.clone()));
        }

        Ok(())
    }

    fn update(&self, project: &ProjectInfo) -> backend::services::container::Result<()> {
        let data = self.data.lock().unwrap();
        let state = data.get(&project.name).unwrap();

        if !*state {
            return Err(ContainerServiceError::AlreadyStopped(project.name.clone()));
        }

        Ok(())
    }
}

pub fn test_server() -> TestServer {
    let project_service = Arc::new(MockProjectService::default());
    let container_service = Arc::new(MockContainerService::default());
    let app = app(project_service.clone(), container_service.clone());

    TestServer::builder().http_transport().build(app).unwrap()
}
