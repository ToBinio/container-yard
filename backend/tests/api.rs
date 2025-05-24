use common::server::test_server;
use serde_json::json;

mod common;

#[tokio::test]
async fn get_projects() {
    let server = test_server();

    let response = server.get("/projects").await;

    response.assert_status_ok();
    response.assert_json(&json!([
        {
            "name": "test",
            "status": "running"
        },
        {
            "name": "test2",
            "status": "stopped"
        },
        {
            "name": "test3",
            "status": "running"
        }
    ]));
}

#[tokio::test]
async fn get_project_details() {
    let server = test_server();

    let response = server.get("/projects/test2").await;

    response.assert_status_ok();
    response.assert_json(&json!({
        "name": "test2",
        "status": "stopped",
        "files": ["compose.yml", ".env"]
    }));
}

#[tokio::test]
async fn get_project_details_no_env() {
    let server = test_server();

    let response = server.get("/projects/test").await;

    response.assert_status_ok();
    response.assert_json(&json!({
        "name": "test",
        "status": "running",
        "files": ["compose.yml"]
    }));
}

#[tokio::test]
async fn get_project_details_unknown() {
    let server = test_server();

    let response = server.get("/projectss/test404").await;

    response.assert_status_not_found();
}

#[tokio::test]
async fn stop_project() {
    let server = test_server();

    let response = server.post("/projects/stop/test").await;

    response.assert_json(&json!({
        "name": "test",
        "status": "stopped",
        "files": ["compose.yml"]
    }));
    response.assert_status_ok();
}

#[tokio::test]
async fn stop_project_unkown() {
    let server = test_server();

    let response = server.post("/projects/start/test404").await;

    response.assert_status_not_found();
}

#[tokio::test]
async fn start_project() {
    let server = test_server();

    let response = server.post("/projects/start/test2").await;

    response.assert_json(&json!({
        "name": "test2",
        "status": "running",
        "files": ["compose.yml", ".env"]
    }));
    response.assert_status_ok();
}

#[tokio::test]
async fn start_project_unkown() {
    let server = test_server();

    let response = server.post("/projects/start/test404").await;

    response.assert_status_not_found();
}

#[tokio::test]
async fn restart_project() {
    let server = test_server();

    let response = server.post("/projects/restart/test").await;

    response.assert_json(&json!({
        "name": "test",
        "status": "running",
        "files": ["compose.yml"]
    }));
    response.assert_status_ok();
}

#[tokio::test]
async fn restart_project_unkown() {
    let server = test_server();

    let response = server.post("/projects/restart/test404").await;

    response.assert_status_not_found();
}

#[tokio::test]
async fn get_project_file() {
    let server = test_server();

    let response = server.get("/projects/test2?file=compose.yml").await;

    response.assert_status_ok();
    response.assert_json(&json!({
        "name": "compose.yml",
        "content": "compose.yml"
    }));
}

#[tokio::test]
async fn get_project_unknown_file() {
    let server = test_server();

    let response = server.get("/projects/test2?file=unknown").await;

    response.assert_status_not_found();
}

#[tokio::test]
async fn update_file_project() {
    let server = test_server();

    let response = server
        .post("/projects/test2?file=compose.yml")
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
    let server = test_server();

    let response = server
        .post("/projects/test2?file=unknown")
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
    let server = test_server();

    let response = server
        .post("/projects/test404?file=compose.yml")
        .json(&json!({
            "content": "newCompose",
        }))
        .await;

    response.assert_status_not_found();
}
