use std::sync::Arc;

use axum::{
    Json, Router,
    extract::{self, FromRef, Path, Query, State},
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
use tower::ServiceBuilder;
use tower_http::{
    cors::{Any, CorsLayer},
    trace::{self, TraceLayer},
};
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
            AppError::Project(error) => error.into_response(),
            AppError::Container(error) => error.into_response(),
        }
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
    let cors_layer = CorsLayer::new()
        .allow_headers(Any)
        .allow_origin(Any)
        .allow_methods(Any);

    Router::new()
        .route("/projects", get(get_all_projects))
        .route("/projects/{project_name}", get(get_project_details))
        .route("/projects/{project_name}", post(post_update_project_file))
        .route("/projects/stop/{project_name}", post(post_stop_project))
        .route("/projects/start/{project_name}", post(post_start_project))
        .route(
            "/projects/restart/{project_name}",
            post(post_restart_project),
        )
        .with_state(AppState {
            project_service,
            container_service,
        })
        .layer(ServiceBuilder::new().layer(cors_layer))
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
    let is_online = container_service.is_online(project_info)?;
    let status = if is_online { "running" } else { "stopped" };

    let files = project_service.files(project_info)?;

    let json = json!({
        "name": project_info.name,
        "status": status,
        "files": files
    });

    Ok(json)
}

#[derive(Deserialize)]
struct FileQuery {
    file: Option<String>,
}

async fn get_project_details(
    State(project_service): State<Arc<dyn ProjectServiceTrait>>,
    State(container_service): State<Arc<dyn ContainerServiceTrait>>,
    Path(project_name): Path<String>,
    Query(query): Query<FileQuery>,
) -> Result<impl IntoResponse, AppError> {
    let project_info = project_service.project(&project_name)?;

    if let Some(file) = query.file {
        let content = project_service.read_file(&project_info, &file)?;

        return Ok(Json(json!({
            "name": file,
            "content": content,
        }))
        .into_response());
    }

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

async fn post_restart_project(
    State(project_service): State<Arc<dyn ProjectServiceTrait>>,
    State(container_service): State<Arc<dyn ContainerServiceTrait>>,
    Path(project_name): Path<String>,
) -> Result<impl IntoResponse, AppError> {
    let project_info = project_service.project(&project_name)?;

    container_service.pull(&project_info)?;
    container_service.start(&project_info)?;

    let json = project_details(&project_info, project_service, container_service)?;
    Ok(Json(json).into_response())
}

#[derive(Deserialize)]
struct UpdateFile {
    content: String,
}

#[derive(Deserialize)]
struct FileUpdateQuery {
    file: String,
}

async fn post_update_project_file(
    State(project_service): State<Arc<dyn ProjectServiceTrait>>,
    Path(project_name): Path<String>,
    Query(query): Query<FileUpdateQuery>,
    extract::Json(update): extract::Json<UpdateFile>,
) -> Result<impl IntoResponse, AppError> {
    let project_info = project_service.project(&project_name)?;
    let content = project_service.update_file(&project_info, &query.file, &update.content)?;

    Ok(Json(json!({
        "name": query.file,
        "content": content,
    }))
    .into_response())
}
