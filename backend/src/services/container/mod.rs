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

    #[error("Failed to exec command {0}")]
    FailedToExecCommand(String),

    #[error("Project already Stopped {0}")]
    AlreadyStopped(String),

    #[error("Project already Running {0}")]
    AlreadyRunning(String),
}

impl IntoResponse for ContainerServiceError {
    fn into_response(self) -> Response {
        let (status, message) = match &self {
            ContainerServiceError::NotFound(_) => (StatusCode::NOT_FOUND, self.to_string()),
            ContainerServiceError::AlreadyStopped(_) => (StatusCode::BAD_REQUEST, self.to_string()),
            ContainerServiceError::AlreadyRunning(_) => (StatusCode::BAD_REQUEST, self.to_string()),
            ContainerServiceError::FailedToExecCommand(_) => {
                (StatusCode::INTERNAL_SERVER_ERROR, self.to_string())
            }
        };

        let body = Json(json!({ "error": message }));
        (status, body).into_response()
    }
}

pub trait ContainerServiceTrait: Send + Sync {
    fn are_online(&self, projects: &[ProjectInfo]) -> Result<Vec<bool>>;
    fn is_online(&self, project: &ProjectInfo) -> Result<bool>;
    fn stop(&self, project: &ProjectInfo) -> Result<()>;
    fn start(&self, project: &ProjectInfo) -> Result<()>;
    fn update(&self, project: &ProjectInfo) -> Result<()>;
}
