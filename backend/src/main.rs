use std::sync::Arc;

use backend::{
    app,
    services::{container::service::ContainerService, project::service::ProjectService},
};
use tracing::info;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8081").await.unwrap();

    info!("listening on {}", listener.local_addr().unwrap());

    axum::serve(
        listener,
        app(
            Arc::new(ProjectService::new("./tests/projects/".into())),
            Arc::new(ContainerService),
        ),
    )
    .await
    .unwrap();
}
