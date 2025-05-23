use std::path::PathBuf;

use axum::{Json, http::StatusCode, response::IntoResponse, response::Response};
use serde::{Deserialize, Serialize};
use serde_json::json;
use thiserror::Error;

pub mod service;

pub type Result<T> = core::result::Result<T, Error>;
pub type Error = ProjectServiceError;

#[derive(Error, Debug, PartialEq)]
pub enum ProjectServiceError {
    #[error("Could not find Project {0}")]
    NotFound(String),
}

impl IntoResponse for ProjectServiceError {
    fn into_response(self) -> Response {
        let (status, message) = match &self {
            ProjectServiceError::NotFound(_) => (StatusCode::NOT_FOUND, self.to_string()),
        };

        let body = Json(json!({ "error": message }));
        (status, body).into_response()
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct ProjectInfo {
    pub name: String,
    pub dir: PathBuf,
}

pub trait ProjectServiceTrait: Send + Sync {
    fn all_projects(&self) -> Result<Vec<ProjectInfo>>;
    fn project(&self, name: &str) -> Result<ProjectInfo>;
    fn compose(&self, project: &ProjectInfo) -> Result<String>;
    fn update_compose(&self, project: &ProjectInfo, compose: String) -> Result<String>;
    fn env(&self, project: &ProjectInfo) -> Result<Option<String>>;
    fn update_env(&self, project: &ProjectInfo, env: String) -> Result<String>;
}
