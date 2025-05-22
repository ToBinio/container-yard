use std::{clone, path::PathBuf, sync::Arc};

use axum_test::TestServer;
use backend::{
    app,
    services::{
        container::ContainerServiceTrait,
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
    data: Vec<Project>,
}

impl Default for MockProjectService {
    fn default() -> Self {
        MockProjectService {
            data: vec![
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
            ],
        }
    }
}

impl ProjectServiceTrait for MockProjectService {
    fn all_projects(
        &self,
    ) -> backend::services::project::Result<Vec<backend::services::project::ProjectInfo>> {
        Ok(self
            .data
            .iter()
            .map(|data| ProjectInfo {
                name: data.name.clone(),
                dir: data.dir.clone(),
            })
            .collect())
    }

    fn project(
        &self,
        name: String,
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
        let project = self
            .data
            .iter()
            .find(|data| data.name == project.name)
            .unwrap();
        Ok(project.compose.clone())
    }

    fn env(
        &self,
        project: &backend::services::project::ProjectInfo,
    ) -> backend::services::project::Result<Option<String>> {
        let project = self
            .data
            .iter()
            .find(|data| data.name == project.name)
            .unwrap();
        Ok(project.env.clone())
    }
}

pub struct MockContainerService;

impl ContainerServiceTrait for MockContainerService {
    fn are_online(
        &self,
        projects: &Vec<backend::services::project::ProjectInfo>,
    ) -> backend::services::container::Result<Vec<bool>> {
        let result: Result<Vec<_>, _> = projects
            .iter()
            .map(|project| self.is_online(project))
            .collect();

        return result;
    }

    fn is_online(
        &self,
        project: &backend::services::project::ProjectInfo,
    ) -> backend::services::container::Result<bool> {
        let result = match project.name.as_str() {
            "test" => true,
            "test2" => false,
            "test3" => true,
            _ => false,
        };

        Ok(result)
    }
}

pub fn test_server() -> TestServer {
    let project_service = Arc::new(MockProjectService::default());
    let container_service = Arc::new(MockContainerService);
    let app = app(project_service.clone(), container_service.clone());

    TestServer::builder().http_transport().build(app).unwrap()
}
