use std::sync::Arc;

use backend::{app, services::project::ProjectService};
use tracing::info;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();

    info!("listening on {}", listener.local_addr().unwrap());

    axum::serve(listener, app(Arc::new(ProjectService::default())))
        .await
        .unwrap();
}
