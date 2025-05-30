use std::{env, sync::Arc};

use backend::{
    AdminAuth, Keys, app,
    services::{container::service::ContainerService, project::service::ProjectService},
};
use tracing::{info, warn};

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();

    dotenv::dotenv().ok();

    let port = env::var("PORT").unwrap_or_else(|_| "8081".to_string());
    let project_dir = env::var("PROJECT_DIR").unwrap_or_else(|_| "./tests/projects/".to_string());
    let secret = env::var("SECRET").unwrap_or_else(|_| {
        warn!("no secret was set!");
        "noSecret".to_string()
    });
    let admin_name = env::var("ADMIN_NAME").unwrap_or_else(|_| {
        warn!("no admin name was set!");
        "admin".to_string()
    });
    let admin_password = env::var("ADMIN_PASSWORD").unwrap_or_else(|_| {
        warn!("no admin password was set!");
        "password".to_string()
    });

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
            Keys::new(secret.as_bytes()),
            AdminAuth {
                name: admin_name,
                password: admin_password,
            },
        ),
    )
    .await
    .unwrap();
}
