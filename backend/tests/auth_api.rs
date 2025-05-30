use common::server::{auth_test_server, test_server};
use serde_json::json;

mod common;

#[tokio::test]
async fn authenticate() {
    let server = test_server();

    let response = server
        .post("/auth")
        .json(&json!({
            "user": "admin",
            "pw": "password",
        }))
        .await;

    let json: serde_json::Value = response.json();
    assert!(json.get("token").is_some(), "Response missing 'token'");

    response.assert_status_ok();
}

#[tokio::test]
async fn authenticate_missing_data() {
    let server = test_server();

    let response = server.post("/auth").json(&json!({})).await;

    response.assert_status_unprocessable_entity();
}

#[tokio::test]
async fn authenticate_invalid_data() {
    let server = test_server();

    let response = server
        .post("/auth")
        .json(&json!({
            "user": "admin",
            "pw": "wrong",
        }))
        .await;

    response.assert_status_unauthorized();
}

#[tokio::test]
async fn validate() {
    let (server, token) = auth_test_server().await;

    let response = server
        .get("/auth/validate")
        .authorization_bearer(token)
        .await;

    response.assert_status_ok();
}

#[tokio::test]
async fn validate_no_token() {
    let server = test_server();

    let response = server.get("/auth/validate").await;

    response.assert_status_unauthorized();
}

#[tokio::test]
async fn validate_invalid_token() {
    let server = test_server();

    let response = server
        .get("/auth/validate")
        .authorization_bearer("invalidToken")
        .await;

    response.assert_status_unauthorized();
}
