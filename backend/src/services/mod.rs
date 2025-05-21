use std::path::PathBuf;

use serde::{Deserialize, Serialize};

pub mod project;

#[derive(Serialize, Deserialize, Debug)]
pub struct ProjectInfo {
    name: String,
    dir: PathBuf,
}

pub trait ProjectServiceTrait: Send + Sync {
    fn all_projects(&self) -> Result<Vec<ProjectInfo>, String>;
    fn compose(&self, project: &ProjectInfo) -> Result<String, String>;
    fn env(&self, project: &ProjectInfo) -> Result<String, String>;
}
