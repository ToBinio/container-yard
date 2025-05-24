use std::process::Command;

use itertools::Itertools;

use crate::services::container::ContainerServiceError;

use super::{ContainerServiceTrait, ProjectInfo};

#[derive(Default)]
pub struct ContainerService;

impl ContainerService {
    fn exec_docker_command(&self, project: &ProjectInfo, args: &[&str]) -> super::Result<()> {
        let output = Command::new("docker")
            .current_dir(&project.dir)
            .args(args)
            .output()
            .map_err(|err| ContainerServiceError::FailedToExecCommand(err.to_string()))?;

        if !output.status.success() {
            return Err(ContainerServiceError::FailedToExecCommand(format!(
                "{:?}",
                output
            )));
        }

        Ok(())
    }
}

impl ContainerServiceTrait for ContainerService {
    fn are_online(&self, projects: &[ProjectInfo]) -> super::Result<Vec<bool>> {
        let output = Command::new("docker")
            .arg("compose")
            .arg("ls")
            .arg("-q")
            .output()
            .map_err(|err| ContainerServiceError::FailedToExecCommand(err.to_string()))?;

        if !output.status.success() {
            return Err(ContainerServiceError::FailedToExecCommand(format!(
                "{:?}",
                output
            )));
        }

        let active_projects = String::from_utf8_lossy(&output.stdout)
            .lines()
            .map(|str| str.to_string())
            .collect_vec();

        let active = projects
            .iter()
            .map(|project_info| active_projects.contains(&project_info.name))
            .collect_vec();

        Ok(active)
    }

    fn is_online(&self, project: &ProjectInfo) -> super::Result<bool> {
        Ok(*self.are_online(&[(*project).clone()])?.first().unwrap())
    }

    fn stop(&self, project: &ProjectInfo) -> super::Result<()> {
        self.exec_docker_command(project, &["compose", "down"])?;
        Ok(())
    }

    fn start(&self, project: &ProjectInfo) -> super::Result<()> {
        self.exec_docker_command(project, &["compose", "up", "-d"])?;
        Ok(())
    }

    fn pull(&self, project: &ProjectInfo) -> super::Result<()> {
        self.exec_docker_command(project, &["compose", "pull"])?;
        Ok(())
    }
}
