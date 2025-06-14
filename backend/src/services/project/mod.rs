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
    ProjectNotFound(String),

    #[error("A Project with the name {0} does already Exist")]
    ProjectAlreadyExists(String),

    #[error("Failed to delete Project {0}")]
    FailedToDeleteProject(String),

    #[error("Could not find file {file} for {project}")]
    FileNotFound { project: String, file: String },

    #[error("Failed to read directory at {0}")]
    FailedToReadDir(String),

    #[error("Failed to read file at {0}")]
    FailedToReadFile(String),

    #[error("Failed to delete file at {0}")]
    FailedToDeleteFile(String),

    #[error("Cannot access files outside of project dir - tried to access {0}")]
    InvalidFilePath(String),
}

impl IntoResponse for ProjectServiceError {
    fn into_response(self) -> Response {
        let status = match &self {
            ProjectServiceError::ProjectNotFound(_) => StatusCode::NOT_FOUND,
            ProjectServiceError::FileNotFound { .. } => StatusCode::NOT_FOUND,
            ProjectServiceError::FailedToReadDir(_) => StatusCode::INTERNAL_SERVER_ERROR,
            ProjectServiceError::FailedToReadFile(_) => StatusCode::INTERNAL_SERVER_ERROR,
            ProjectServiceError::FailedToDeleteFile(_) => StatusCode::INTERNAL_SERVER_ERROR,
            ProjectServiceError::InvalidFilePath(_) => StatusCode::BAD_REQUEST,
            ProjectServiceError::ProjectAlreadyExists(_) => StatusCode::BAD_REQUEST,
            ProjectServiceError::FailedToDeleteProject(_) => StatusCode::INTERNAL_SERVER_ERROR,
        };

        let body = Json(json!({ "error": self.to_string() }));
        (status, body).into_response()
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, PartialOrd, Eq, Ord, Clone)]
pub struct ProjectInfo {
    pub name: String,
    pub dir: PathBuf,
}

pub trait ProjectServiceTrait: Send + Sync {
    fn all_projects(&self) -> Result<Vec<ProjectInfo>>;
    fn project(&self, name: &str) -> Result<ProjectInfo>;
    fn create(&self, name: &str) -> Result<ProjectInfo>;
    fn delete(&self, project: &ProjectInfo) -> Result<()>;
    fn files(&self, project: &ProjectInfo) -> Result<Vec<String>>;
    fn read_file(&self, project: &ProjectInfo, file: &str) -> Result<String>;
    fn update_file(&self, project: &ProjectInfo, file: &str, content: &str) -> Result<String>;
    fn delete_file(&self, project: &ProjectInfo, file: &str) -> Result<()>;
}
