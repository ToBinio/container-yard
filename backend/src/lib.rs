use std::sync::Arc;

use axum::{
    Json, Router,
    extract::{self, FromRef, Path, State},
    response::{IntoResponse, Response},
    routing::{get, post},
};
use serde::Deserialize;
use serde_json::{Value, json};
use services::{
    container::{ContainerServiceError, ContainerServiceTrait},
    project::{ProjectInfo, ProjectServiceError, ProjectServiceTrait},
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
        .route("/projects/stop/{project_name}", post(post_stop_project))
        .route("/projects/start/{project_name}", post(post_start_project))
        .route("/projects/update/{project_name}", post(post_update_project))
        .route(
            "/projects/compose/update/{project_name}",
            post(post_update_compose_project),
        )
        .route(
            "/projects/env/update/{project_name}",
            post(post_update_env_project),
        )
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

fn project_details(
    project_info: &ProjectInfo,
    project_service: Arc<dyn ProjectServiceTrait>,
    container_service: Arc<dyn ContainerServiceTrait>,
) -> Result<serde_json::Value, AppError> {
    let is_online = container_service.is_online(&project_info)?;
    let status = if is_online { "running" } else { "stopped" };

    let compose = project_service.compose(&project_info)?;
    let env = project_service.env(&project_info)?;

    let json = if let Some(env) = env {
        json!({
            "name": project_info.name,
            "status": status,
            "compose": compose,
            "env": env
        })
    } else {
        json!({
            "name": project_info.name,
            "status": status,
            "compose": compose,
        })
    };

    return Ok(json);
}

async fn get_project_details(
    State(project_service): State<Arc<dyn ProjectServiceTrait>>,
    State(container_service): State<Arc<dyn ContainerServiceTrait>>,
    Path(project_name): Path<String>,
) -> Result<impl IntoResponse, AppError> {
    let project_info = project_service.project(&project_name)?;

    let json = project_details(&project_info, project_service, container_service)?;
    Ok(Json(json).into_response())
}

async fn post_stop_project(
    State(project_service): State<Arc<dyn ProjectServiceTrait>>,
    State(container_service): State<Arc<dyn ContainerServiceTrait>>,
    Path(project_name): Path<String>,
) -> Result<impl IntoResponse, AppError> {
    let project_info = project_service.project(&project_name)?;

    container_service.stop(&project_info)?;

    let json = project_details(&project_info, project_service, container_service)?;
    Ok(Json(json).into_response())
}

async fn post_start_project(
    State(project_service): State<Arc<dyn ProjectServiceTrait>>,
    State(container_service): State<Arc<dyn ContainerServiceTrait>>,
    Path(project_name): Path<String>,
) -> Result<impl IntoResponse, AppError> {
    let project_info = project_service.project(&project_name)?;

    container_service.start(&project_info)?;

    let json = project_details(&project_info, project_service, container_service)?;
    Ok(Json(json).into_response())
}

async fn post_update_project(
    State(project_service): State<Arc<dyn ProjectServiceTrait>>,
    State(container_service): State<Arc<dyn ContainerServiceTrait>>,
    Path(project_name): Path<String>,
) -> Result<impl IntoResponse, AppError> {
    let project_info = project_service.project(&project_name)?;

    container_service.update(&project_info)?;

    let json = project_details(&project_info, project_service, container_service)?;
    Ok(Json(json).into_response())
}

#[derive(Deserialize)]
struct UpdateCompose {
    compose: String,
}

async fn post_update_compose_project(
    State(project_service): State<Arc<dyn ProjectServiceTrait>>,
    State(container_service): State<Arc<dyn ContainerServiceTrait>>,
    Path(project_name): Path<String>,
    extract::Json(update): extract::Json<UpdateCompose>,
) -> Result<impl IntoResponse, AppError> {
    let project_info = project_service.project(&project_name)?;

    project_service.update_compose(&project_info, update.compose)?;

    let json = project_details(&project_info, project_service, container_service)?;
    Ok(Json(json).into_response())
}

#[derive(Deserialize)]
struct UpdateEnv {
    env: String,
}

async fn post_update_env_project(
    State(project_service): State<Arc<dyn ProjectServiceTrait>>,
    State(container_service): State<Arc<dyn ContainerServiceTrait>>,
    Path(project_name): Path<String>,
    extract::Json(update): extract::Json<UpdateEnv>,
) -> Result<impl IntoResponse, AppError> {
    let project_info = project_service.project(&project_name)?;

    project_service.update_env(&project_info, update.env)?;

    let json = project_details(&project_info, project_service, container_service)?;
    Ok(Json(json).into_response())
}
