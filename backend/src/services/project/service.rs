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
            return Err(ProjectServiceError::NotFound(name.to_string()));
        }

        Ok(ProjectInfo {
            name: name.to_string(),
            dir: path,
        })
    }

    // fn compose(&self, project: &ProjectInfo) -> super::Result<String> {
    //     let path = project.dir.join("compose.yml");

    //     fs::read_to_string(&path)
    //         .inspect_err(|err| error!("{}", err))
    //         .map_err(|_| ProjectServiceError::NotFound(project.name.to_string()))
    // }

    // fn update_compose(&self, project: &ProjectInfo, compose: String) -> super::Result<String> {
    //     let path = project.dir.join("compose.yml");
    //     let mut file = File::create(&path)
    //         .inspect_err(|err| error!("{}", err))
    //         .map_err(|_| ProjectServiceError::NotFound(project.name.to_string()))?;

    //     file.write_all(compose.as_bytes())
    //         .inspect_err(|err| error!("{}", err))
    //         .map_err(|_| ProjectServiceError::FailedToReadDir(format!("{:?}", path)))?;

    //     Ok(compose)
    // }

    // fn env(&self, project: &ProjectInfo) -> super::Result<Option<String>> {
    //     let path = project.dir.join(".env");

    //     let exist = fs::exists(&path)
    //         .inspect_err(|err| error!("{}", err))
    //         .map_err(|_| ProjectServiceError::FailedToReadDir(format!("{:?}", path)))?;

    //     if !exist {
    //         return Ok(None);
    //     }

    //     Ok(Some(
    //         fs::read_to_string(&path)
    //             .inspect_err(|err| error!("{}", err))
    //             .map_err(|_| ProjectServiceError::NotFound(project.name.to_string()))?,
    //     ))
    // }

    // fn update_env(&self, project: &ProjectInfo, env: String) -> super::Result<String> {
    //     let path = project.dir.join(".env");
    //     let mut file = File::create(&path)
    //         .inspect_err(|err| error!("{}", err))
    //         .map_err(|_| ProjectServiceError::NotFound(project.name.to_string()))?;

    //     file.write_all(env.as_bytes())
    //         .inspect_err(|err| error!("{}", err))
    //         .map_err(|_| ProjectServiceError::FailedToReadDir(format!("{:?}", path)))?;

    //     Ok(env)
    // }

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
        let path = project.dir.join(file);

        let exist = fs::exists(&path)
            .inspect_err(|err| error!("{}", err))
            .map_err(|_| ProjectServiceError::FailedToReadDir(format!("{:?}", path)))?;

        if !exist {
            return Err(ProjectServiceError::NotFound(format!("{:?}", path)));
        }

        let content = fs::read_to_string(&path)
            .inspect_err(|err| error!("{}", err))
            .map_err(|_| ProjectServiceError::NotFound(project.name.to_string()))?;

        Ok(content)
    }

    fn update_file(
        &self,
        project: &ProjectInfo,
        file: &str,
        content: &str,
    ) -> super::Result<String> {
        let path = project.dir.join(file);
        let mut file = File::create(&path)
            .inspect_err(|err| error!("{}", err))
            .map_err(|_| ProjectServiceError::NotFound(project.name.to_string()))?;

        file.write_all(content.as_bytes())
            .inspect_err(|err| error!("{}", err))
            .map_err(|_| ProjectServiceError::FailedToReadDir(format!("{:?}", path)))?;

        Ok(content.to_string())
    }
}
