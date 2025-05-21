use std::sync::Arc;

use axum::{
    Json, Router,
    extract::{FromRef, Path, State},
    response::{IntoResponse, Response},
    routing::get,
};
use serde_json::{Value, json};
use services::{
    container::{ContainerServiceError, ContainerServiceTrait},
    project::{ProjectServiceError, ProjectServiceTrait},
};
use thiserror::Error;
use tower_http::trace::{self, TraceLayer};
use tracing::{Level, error};

pub mod services;

#[derive(Error, Debug)]
pub enum AppError {
    #[error(transparent)]
    Project(#[from] ProjectServiceError),

    #[error(transparent)]
    Container(#[from] ContainerServiceError),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        match self {
            AppError::Project(error) => return error.into_response(),
            AppError::Container(error) => return error.into_response(),
        };
    }
}

#[derive(Clone)]
struct AppState {
    project_service: Arc<dyn ProjectServiceTrait>,
    container_service: Arc<dyn ContainerServiceTrait>,
}

impl FromRef<AppState> for Arc<dyn ProjectServiceTrait> {
    fn from_ref(input: &AppState) -> Self {
        input.project_service.clone()
    }
}

impl FromRef<AppState> for Arc<dyn ContainerServiceTrait> {
    fn from_ref(input: &AppState) -> Self {
        input.container_service.clone()
    }
}

pub fn app(
    project_service: Arc<dyn ProjectServiceTrait>,
    container_service: Arc<dyn ContainerServiceTrait>,
) -> Router {
    Router::new()
        .route("/projects", get(get_all_projects))
        .route("/projects/{project_name}", get(get_project_details))
        .with_state(AppState {
            project_service: project_service,
            container_service: container_service,
        })
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(trace::DefaultMakeSpan::new().level(Level::INFO))
                .on_response(trace::DefaultOnResponse::new().level(Level::INFO)),
        )
}

async fn get_all_projects(
    State(project_service): State<Arc<dyn ProjectServiceTrait>>,
    State(container_service): State<Arc<dyn ContainerServiceTrait>>,
) -> Result<impl IntoResponse, AppError> {
    let projects = project_service.all_projects()?;
    let are_online = container_service.are_online(&projects)?;

    let objects: Vec<Value> = projects
        .into_iter()
        .zip(are_online.into_iter())
        .map(|(project, is_online)| {
            let status = if is_online { "running" } else { "stopped" };

            json!({
                "name": project.name,
                "status": status
            })
        })
        .collect();

    Ok(Json(json!(objects)))
}

async fn get_project_details(
    State(project_service): State<Arc<dyn ProjectServiceTrait>>,
    Path(project_name): Path<String>,
) -> Result<impl IntoResponse, AppError> {
    let project_info = project_service.project(project_name)?;

    Ok(Json(json!({
        "name": project_info.name
    }))
    .into_response())
}
