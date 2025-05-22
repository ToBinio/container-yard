use serde_json::json;

mod common;

#[tokio::test]
async fn get_projects() {
    let server = common::test_server();

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
    let server = common::test_server();

    let response = server.get("/projects/test2").await;

    response.assert_status_ok();
    response.assert_json(&json!({
        "name": "test2",
        "status": "stopped",
        "compose": "compose.yml",
        "env": ".env"
    }));
}

#[tokio::test]
async fn get_project_details_no_env() {
    let server = common::test_server();

    let response = server.get("/projects/test").await;

    response.assert_status_ok();
    response.assert_json(&json!({
        "name": "test",
        "status": "running",
        "compose": "compose.yml"
    }));
}

#[tokio::test]
async fn get_project_details_unknown() {
    let server = common::test_server();

    let response = server.get("/projectss/test404").await;

    response.assert_status_not_found();
}

#[tokio::test]
async fn stop_project() {
    let server = common::test_server();

    let response = server.post("/projects/stop/test").await;

    response.assert_json(&json!({
        "name": "test",
        "status": "stopped",
        "compose": "compose.yml"
    }));
    response.assert_status_ok();
}

#[tokio::test]
async fn stop_project_already_stopped() {
    let server = common::test_server();

    let response = server.post("/projects/stop/test2").await;

    response.assert_status_bad_request();
}

#[tokio::test]
async fn stop_project_unkown() {
    let server = common::test_server();

    let response = server.post("/projects/start/test404").await;

    response.assert_status_not_found();
}

#[tokio::test]
async fn start_project() {
    let server = common::test_server();

    let response = server.post("/projects/start/test2").await;

    response.assert_json(&json!({
        "name": "test2",
        "status": "running",
        "compose": "compose.yml",
        "env": ".env"
    }));
    response.assert_status_ok();
}

#[tokio::test]
async fn start_project_already_started() {
    let server = common::test_server();

    let response = server.post("/projects/start/test").await;

    response.assert_status_bad_request();
}

#[tokio::test]
async fn start_project_unkown() {
    let server = common::test_server();

    let response = server.post("/projects/start/test404").await;

    response.assert_status_not_found();
}

#[tokio::test]
async fn update_project() {
    let server = common::test_server();

    let response = server.post("/projects/update/test").await;

    response.assert_json(&json!({
        "name": "test",
        "status": "running",
        "compose": "compose.yml"
    }));
    response.assert_status_ok();
}

#[tokio::test]
async fn update_project_already_stopped() {
    let server = common::test_server();

    let response = server.post("/projects/update/test2").await;

    response.assert_status_bad_request();
}

#[tokio::test]
async fn update_project_unkown() {
    let server = common::test_server();

    let response = server.post("/projects/update/test404").await;

    response.assert_status_not_found();
}

#[tokio::test]
async fn update_compose_project() {
    let server = common::test_server();

    let response = server
        .post("/projects/compose/update/test")
        .json(&json!({
            "compose": "newCompose",
        }))
        .await;

    response.assert_json(&json!({
        "name": "test",
        "status": "running",
        "compose": "newCompose"
    }));
    response.assert_status_ok();
}

#[tokio::test]
async fn update_compose_project_no_compose() {
    let server = common::test_server();

    let response = server
        .post("/projects/compose/update/test2")
        .json(&json!({}))
        .await;

    response.assert_status_unprocessable_entity();
}

#[tokio::test]
async fn update_compose_project_unkown() {
    let server = common::test_server();

    let response = server
        .post("/projects/compose/update/test404")
        .json(&json!({
            "compose": "newCompose",
        }))
        .await;

    response.assert_status_not_found();
}

#[tokio::test]
async fn update_env_project() {
    let server = common::test_server();

    let response = server
        .post("/projects/env/update/test2")
        .json(&json!({
            "env": "newEnv",
        }))
        .await;

    response.assert_json(&json!({
        "name": "test2",
        "status": "stopped",
        "compose": "compose.yml",
        "env": "newEnv"
    }));
    response.assert_status_ok();
}

#[tokio::test]
async fn update_env_project_no_env_yet() {
    let server = common::test_server();

    let response = server
        .post("/projects/env/update/test")
        .json(&json!({
            "env": "newEnv",
        }))
        .await;

    response.assert_json(&json!({
        "name": "test",
        "status": "running",
        "compose": "compose.yml",
        "env": "newEnv"
    }));
    response.assert_status_ok();
}

#[tokio::test]
async fn update_env_project_no_env() {
    let server = common::test_server();

    let response = server
        .post("/projects/env/update/test2")
        .json(&json!({}))
        .await;

    response.assert_status_unprocessable_entity();
}

#[tokio::test]
async fn update_env_project_unkown() {
    let server = common::test_server();

    let response = server
        .post("/projects/env/update/test404")
        .json(&json!({
            "env": "newEnv",
        }))
        .await;

    response.assert_status_not_found();
}
