#![allow(dead_code)]

use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use axum_test::TestServer;
use backend::{
    AdminAuth, Keys, app,
    services::{container::ContainerServiceTrait, project::ProjectInfo},
};
use cookie::Cookie;
use serde_json::json;
use tempfile::TempDir;

use crate::common::project_service::test_project_service;

pub struct MockContainerService {
    data: Arc<Mutex<HashMap<String, bool>>>,
}

impl Default for MockContainerService {
    fn default() -> Self {
        let mut map = HashMap::new();

        map.insert("project1".to_string(), true);
        map.insert("project2".to_string(), false);
        map.insert("project3".to_string(), true);

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
        Ok(*self
            .data
            .lock()
            .unwrap()
            .get(&project.name)
            .unwrap_or(&false))
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

pub fn test_server() -> (TempDir, TestServer) {
    let (dir, project_service) = test_project_service();

    let project_service = Arc::new(project_service);
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

    (
        dir,
        TestServer::builder().http_transport().build(app).unwrap(),
    )
}

pub async fn auth_test_server() -> (TempDir, TestServer, String) {
    let (dir, mut server) = test_server();

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

    (dir, server, token)
}
