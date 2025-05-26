use std::{env, sync::Arc};

use backend::{
    app,
    services::{container::service::ContainerService, project::service::ProjectService},
};
use tracing::info;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    let port = env::var("PORT").unwrap_or_else(|_| "8081".to_string());
    let project_dir = env::var("PROJECT_DIR").unwrap_or_else(|_| "./tests/projects/".to_string());

    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();

    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", port))
        .await
        .unwrap();

    info!("listening on {}", listener.local_addr().unwrap());
    info!("using project path '{}'", project_dir.clone());

    axum::serve(
        listener,
        app(
            Arc::new(ProjectService::new(project_dir.into())),
            Arc::new(ContainerService),
        ),
    )
    .await
    .unwrap();
}
