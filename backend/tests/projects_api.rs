use common::server::{auth_test_server, test_server};
use serde_json::json;

mod common;

#[tokio::test]
async fn require_login() {
    let (_dir, server) = test_server();

    let responses = vec![
        server.get("/projects").await,
        server.get("/projects/project1").await,
        server.post("/projects/create/project1").await,
        server.get("/projects/project1?file=compose.yml").await,
        server.delete("/projects/project1?file=compose.yml").await,
        server.post("/projects/stop/project1").await,
        server.post("/projects/start/project1").await,
        server.post("/projects/restart/project1").await,
        server
            .post("/projects/project1?file=compose.yml")
            .json(&json!({
                "content": "newCompose",
            }))
            .await,
    ];

    for response in responses {
        response.assert_status_unauthorized()
    }
}

#[tokio::test]
async fn get_projects() {
    let (_dir, server, _token) = auth_test_server().await;

    let response = server.get("/projects").await;

    response.assert_status_ok();
    response.assert_json(&json!([
        {
            "name": "project1",
            "status": "running"
        },
        {
            "name": "project2",
            "status": "stopped"
        },
        {
            "name": "project3",
            "status": "running"
        }
    ]));
}

#[tokio::test]
async fn get_project_details() {
    let (_dir, server, _token) = auth_test_server().await;

    let response = server.get("/projects/project1").await;

    response.assert_status_ok();
    response.assert_json(&json!({
        "name": "project1",
        "status": "running",
        "files": [".env", "compose.yml"]
    }));
}

#[tokio::test]
async fn get_project_details_no_files() {
    let (_dir, server, _token) = auth_test_server().await;

    let response = server.get("/projects/project2").await;

    response.assert_status_ok();
    response.assert_json(&json!({
        "name": "project2",
        "status": "stopped",
        "files": []
    }));
}

#[tokio::test]
async fn get_project_details_unknown() {
    let (_dir, server, _token) = auth_test_server().await;

    let response = server.get("/projectss/project404").await;

    response.assert_status_not_found();
}

#[tokio::test]
async fn stop_project() {
    let (_dir, server, _token) = auth_test_server().await;

    let response = server.post("/projects/stop/project1").await;

    response.assert_json(&json!({
        "name": "project1",
        "status": "stopped",
        "files": [".env", "compose.yml"]
    }));
    response.assert_status_ok();
}

#[tokio::test]
async fn stop_project_unkown() {
    let (_dir, server, _token) = auth_test_server().await;

    let response = server.post("/projects/start/project404").await;

    response.assert_status_not_found();
}

#[tokio::test]
async fn start_project() {
    let (_dir, server, _token) = auth_test_server().await;

    let response = server.post("/projects/start/project2").await;

    response.assert_json(&json!({
        "name": "project2",
        "status": "running",
        "files": []
    }));
    response.assert_status_ok();
}

#[tokio::test]
async fn start_project_unkown() {
    let (_dir, server, _token) = auth_test_server().await;

    let response = server.post("/projects/start/project404").await;

    response.assert_status_not_found();
}

#[tokio::test]
async fn restart_project() {
    let (_dir, server, _token) = auth_test_server().await;

    let response = server.post("/projects/restart/project1").await;

    response.assert_json(&json!({
        "name": "project1",
        "status": "running",
        "files": [".env", "compose.yml"]
    }));
    response.assert_status_ok();
}

#[tokio::test]
async fn restart_project_unkown() {
    let (_dir, server, _token) = auth_test_server().await;

    let response = server.post("/projects/restart/project404").await;

    response.assert_status_not_found();
}

#[tokio::test]
async fn get_project_file() {
    let (_dir, server, _token) = auth_test_server().await;

    let response = server.get("/projects/project1?file=compose.yml").await;

    response.assert_status_ok();
    response.assert_json(&json!({
        "name": "compose.yml",
        "content": "compose.yml"
    }));
}

#[tokio::test]
async fn get_project_unknown_file() {
    let (_dir, server, _token) = auth_test_server().await;

    let response = server.get("/projects/project1?file=unknown").await;

    response.assert_status_not_found();
}

#[tokio::test]
async fn update_file_project() {
    let (_dir, server, _token) = auth_test_server().await;

    let response = server
        .post("/projects/project1?file=compose.yml")
        .json(&json!({
            "content": "newCompose",
        }))
        .await;

    response.assert_json(&json!({
        "name": "compose.yml",
        "content": "newCompose"
    }));
    response.assert_status_ok();
}

#[tokio::test]
async fn update_unknown_file_project() {
    let (_dir, server, _token) = auth_test_server().await;

    let response = server
        .post("/projects/project1?file=unknown")
        .json(&json!({
            "content": "content",
        }))
        .await;

    response.assert_json(&json!({
        "name": "unknown",
        "content": "content"
    }));
    response.assert_status_ok();
}

#[tokio::test]
async fn update_file_unknown_project() {
    let (_dir, server, _token) = auth_test_server().await;

    let response = server
        .post("/projects/project404?file=compose.yml")
        .json(&json!({
            "content": "newCompose",
        }))
        .await;

    response.assert_status_not_found();
}

#[tokio::test]
async fn create_new_project() {
    let (_dir, server, _token) = auth_test_server().await;

    let response = server.post("/projects/create/newProject").await;

    response.assert_json(&json!({
        "name": "newProject",
        "status": "stopped",
        "files": ["compose.yml"]
    }));
    response.assert_status_ok();
}

#[tokio::test]
async fn create_already_existing_project() {
    let (_dir, server, _token) = auth_test_server().await;

    let response = server.post("/projects/create/project1").await;

    response.assert_status_bad_request();
}

#[tokio::test]
async fn delete_file_project() {
    let (_dir, server, _token) = auth_test_server().await;

    let response = server.delete("/projects/project1?file=compose.yml").await;

    response.assert_json(&json!({
        "name": "compose.yml",
        "content": "compose.yml"
    }));
    response.assert_status_ok();
}

#[tokio::test]
async fn delete_unknown_file_project() {
    let (_dir, server, _token) = auth_test_server().await;

    let response = server.delete("/projects/project1?file=unknown.txt").await;

    response.assert_status_not_found();
}

#[tokio::test]
async fn delete_missing_file_project() {
    let (_dir, server, _token) = auth_test_server().await;

    let response = server.delete("/projects/project404").await;

    response.assert_status_bad_request();
}
