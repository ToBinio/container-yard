use std::sync::Arc;

use axum_test::TestServer;
use backend::{app, services::ProjectServiceTrait};

pub struct MockProjectService;

impl ProjectServiceTrait for MockProjectService {
    fn all_projects(&self) -> Result<Vec<backend::services::ProjectInfo>, String> {
        todo!()
    }

    fn compose(&self, project: &backend::services::ProjectInfo) -> Result<String, String> {
        todo!()
    }

    fn env(&self, project: &backend::services::ProjectInfo) -> Result<String, String> {
        todo!()
    }
}

pub fn test_server() -> TestServer {
    let podman_service = Arc::new(MockProjectService);
    let app = app(podman_service.clone());

    TestServer::builder().http_transport().build(app).unwrap()
}
