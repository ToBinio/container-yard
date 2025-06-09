use std::sync::Arc;

use axum::{
    Json, Router,
    extract::{self, Path, Query, State},
    middleware::from_extractor_with_state,
    response::IntoResponse,
    routing::{delete, get, post},
};
use serde::Deserialize;
use serde_json::{Value, json};

use crate::{
    AppError, AppState,
    services::{
        container::ContainerServiceTrait,
        project::{ProjectInfo, ProjectServiceTrait},
    },
};

use super::auth::Claims;

pub fn routes(state: AppState) -> Router {
    Router::new()
        .route("/", get(get_all_projects))
        .route("/{project_name}", get(get_project_details))
        .route("/{project_name}", post(post_update_project_file))
        .route("/{project_name}", delete(delete_project))
        .route("/stop/{project_name}", post(post_stop_project))
        .route("/start/{project_name}", post(post_start_project))
        .route("/restart/{project_name}", post(post_restart_project))
        .route("/create/{project_name}", post(post_create_project))
        .route_layer(from_extractor_with_state::<Claims, _>(state.clone()))
        .with_state(state)
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

async fn delete_project(
    State(project_service): State<Arc<dyn ProjectServiceTrait>>,
    Path(project_name): Path<String>,
    Query(query): Query<FileQuery>,
) -> Result<(), AppError> {
    let project_info = project_service.project(&project_name)?;

    if let Some(file) = query.file {
        project_service.delete_file(&project_info, &file)?;
    } else {
        project_service.delete(&project_info)?;
    }

    Ok(())
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

async fn post_create_project(
    State(project_service): State<Arc<dyn ProjectServiceTrait>>,
    State(container_service): State<Arc<dyn ContainerServiceTrait>>,
    Path(project_name): Path<String>,
) -> Result<impl IntoResponse, AppError> {
    let project_info = project_service.create(&project_name)?;

    let json = project_details(&project_info, project_service, container_service)?;
    Ok(Json(json).into_response())
}
