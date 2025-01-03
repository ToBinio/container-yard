use axum::routing::get;
use axum::{Json, Router};
use serde::{Deserialize, Serialize};
use tower_http::trace::TraceLayer;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    let app = app();

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

fn app() -> Router {
    Router::new()
        .route("/name", get(name))
        .layer(TraceLayer::new_for_http())
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Name {
    name: String,
}

async fn name() -> Json<Name> {
    Json(Name {
        name: "Tobi".to_string(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::body::Body;
    use axum::http::{Request, StatusCode};
    use http_body_util::BodyExt;
    use tower::ServiceExt;

    #[tokio::test]
    async fn name() {
        let app = app();

        let response = app
            .oneshot(Request::builder().uri("/name").body(Body::empty()).unwrap())
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let body = response.into_body().collect().await.unwrap().to_bytes();
        let body = serde_json::from_slice::<Name>(&body[..]).unwrap();
        assert_eq!(
            body,
            Name {
                name: "Tobi".to_string()
            }
        );
    }
}
