use axum::{Json, http::StatusCode, response::IntoResponse, response::Response};
use serde_json::json;
use thiserror::Error;

use super::project::ProjectInfo;

pub mod service;

pub type Result<T> = core::result::Result<T, Error>;
pub type Error = ContainerServiceError;

#[derive(Error, Debug)]
pub enum ContainerServiceError {
    #[error("Could not find Project {0}")]
    NotFound(String),

    #[error("Failed to exec command '{command}' - {error}")]
    FailedToExecCommand { error: String, command: String },
}

impl IntoResponse for ContainerServiceError {
    fn into_response(self) -> Response {
        let status = match &self {
            ContainerServiceError::NotFound(_) => StatusCode::NOT_FOUND,
            ContainerServiceError::FailedToExecCommand { .. } => StatusCode::INTERNAL_SERVER_ERROR,
        };

        let body = Json(json!({ "error": self.to_string() }));
        (status, body).into_response()
    }
}

pub trait ContainerServiceTrait: Send + Sync {
    fn are_online(&self, projects: &[ProjectInfo]) -> Result<Vec<bool>>;
    fn is_online(&self, project: &ProjectInfo) -> Result<bool>;
    fn stop(&self, project: &ProjectInfo) -> Result<()>;
    fn start(&self, project: &ProjectInfo) -> Result<()>;
    fn pull(&self, project: &ProjectInfo) -> Result<()>;
}
