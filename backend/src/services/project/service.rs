use std::{
    fs::{self, File},
    io::Write,
    path::PathBuf,
};

use itertools::Itertools;
use tracing::error;

use super::{ProjectInfo, ProjectServiceError, ProjectServiceTrait};

#[derive(Default)]
pub struct ProjectService {
    base_path: PathBuf,
}

impl ProjectService {
    pub fn new(base_path: PathBuf) -> ProjectService {
        Self { base_path }
    }

    pub fn save_file_path(file: &str) -> super::Result<PathBuf> {
        let path = PathBuf::from(file);
        if path.components().count() != 1 {
            return Err(ProjectServiceError::NotProjectFile(file.to_string()));
        }

        Ok(path)
    }
}

impl ProjectServiceTrait for ProjectService {
    fn all_projects(&self) -> super::Result<Vec<ProjectInfo>> {
        let dir = fs::read_dir(&self.base_path)
            .inspect_err(|err| error!("{}", err))
            .map_err(|_| ProjectServiceError::FailedToReadDir(format!("{:?}", self.base_path)))?;

        let projects = dir
            .filter_map(|entry| entry.ok())
            .map(|entry| entry.path())
            .filter(|path| path.is_dir())
            .map(|path| ProjectInfo {
                name: path.file_name().unwrap().to_str().unwrap().to_string(),
                dir: path,
            })
            .sorted()
            .collect();

        Ok(projects)
    }

    fn project(&self, name: &str) -> super::Result<ProjectInfo> {
        let path = self.base_path.join(name);

        let exist = fs::exists(&path)
            .inspect_err(|err| error!("{}", err))
            .map_err(|_| ProjectServiceError::FailedToReadDir(format!("{:?}", path)))?;

        if !exist {
            return Err(ProjectServiceError::ProjectNotFound(name.to_string()));
        }

        Ok(ProjectInfo {
            name: name.to_string(),
            dir: path,
        })
    }

    fn files(&self, project: &ProjectInfo) -> super::Result<Vec<String>> {
        let dir = fs::read_dir(&project.dir)
            .inspect_err(|err| error!("{}", err))
            .map_err(|_| ProjectServiceError::FailedToReadDir(format!("{:?}", project.dir)))?;

        let files = dir
            .filter_map(|entry| entry.ok())
            .map(|entry| entry.path())
            .filter(|path| path.is_file())
            .map(|path| path.file_name().unwrap().to_str().unwrap().to_string())
            .sorted()
            .collect();

        Ok(files)
    }

    fn read_file(&self, project: &ProjectInfo, file: &str) -> super::Result<String> {
        let path = Self::save_file_path(file)?;
        let path = project.dir.join(path);

        let exist = fs::exists(&path)
            .inspect_err(|err| error!("{}", err))
            .map_err(|_| ProjectServiceError::FailedToReadDir(format!("{:?}", path)))?;

        if !exist {
            return Err(ProjectServiceError::FileNotFound {
                project: project.name.to_string(),
                file: format!("{:?}", path),
            });
        }

        let content = fs::read_to_string(&path)
            .inspect_err(|err| error!("{}", err))
            .map_err(|_| ProjectServiceError::FailedToReadDir(format!("{:?}", path)))?;

        Ok(content)
    }

    fn update_file(
        &self,
        project: &ProjectInfo,
        file: &str,
        content: &str,
    ) -> super::Result<String> {
        let path = Self::save_file_path(file)?;
        let path = project.dir.join(path);

        let mut file = File::create(&path)
            .inspect_err(|err| error!("{}", err))
            .map_err(|_| ProjectServiceError::FailedToReadDir(format!("{:?}", path)))?;

        file.write_all(content.as_bytes())
            .inspect_err(|err| error!("{}", err))
            .map_err(|_| ProjectServiceError::FailedToReadDir(format!("{:?}", path)))?;

        Ok(content.to_string())
    }
}
