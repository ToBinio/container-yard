use std::sync::Arc;

use axum_test::TestServer;
use backend::{
    app,
    services::{container::ContainerServiceTrait, project::ProjectServiceTrait},
};
use itertools::rev;

pub struct MockProjectService;

impl ProjectServiceTrait for MockProjectService {
    fn all_projects(
        &self,
    ) -> backend::services::project::Result<Vec<backend::services::project::ProjectInfo>> {
        Ok(vec![
            backend::services::project::ProjectInfo {
                name: "test".to_string(),
                dir: "/path/to/project1".into(),
            },
            backend::services::project::ProjectInfo {
                name: "test2".to_string(),
                dir: "/path/to/project2".into(),
            },
            backend::services::project::ProjectInfo {
                name: "test3".to_string(),
                dir: "/path/to/project2".into(),
            },
        ])
    }

    fn project(
        &self,
        name: String,
    ) -> backend::services::project::Result<backend::services::project::ProjectInfo> {
        todo!()
    }

    fn compose(
        &self,
        project: &backend::services::project::ProjectInfo,
    ) -> backend::services::project::Result<String> {
        todo!()
    }

    fn env(
        &self,
        project: &backend::services::project::ProjectInfo,
    ) -> backend::services::project::Result<String> {
        todo!()
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
    let project_service = Arc::new(MockProjectService);
    let container_service = Arc::new(MockContainerService);
    let app = app(project_service.clone(), container_service.clone());

    TestServer::builder().http_transport().build(app).unwrap()
}
