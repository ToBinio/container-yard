use std::{
    path::PathBuf,
    process::{Command, Output},
};

use itertools::Itertools;

use crate::services::container::ContainerServiceError;

use super::{ContainerServiceTrait, ProjectInfo};

#[derive(Default)]
pub struct ContainerService;

impl ContainerService {
    fn exec_docker_compose_command(
        &self,
        base_dir: Option<&PathBuf>,
        args: &[&str],
    ) -> super::Result<Output> {
        let mut command = Command::new("docker");

        if let Some(path) = base_dir {
            command.current_dir(path);
        };

        let output = command.arg("compose").args(args).output().map_err(|err| {
            ContainerServiceError::FailedToExecCommand {
                command: args.join(" ").to_string(),
                error: err.to_string(),
            }
        })?;

        if !output.status.success() {
            return Err(ContainerServiceError::FailedToExecCommand {
                command: args.join(" ").to_string(),
                error: format!("{:?}", output),
            });
        }

        Ok(output)
    }
}

impl ContainerServiceTrait for ContainerService {
    fn are_online(&self, projects: &[ProjectInfo]) -> super::Result<Vec<bool>> {
        let output = self.exec_docker_compose_command(None, &["ls", "-q"])?;

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
        self.exec_docker_compose_command(Some(&project.dir), &["down"])?;
        Ok(())
    }

    fn start(&self, project: &ProjectInfo) -> super::Result<()> {
        self.exec_docker_compose_command(Some(&project.dir), &["up", "-d"])?;
        Ok(())
    }

    fn pull(&self, project: &ProjectInfo) -> super::Result<()> {
        self.exec_docker_compose_command(Some(&project.dir), &["pull"])?;
        Ok(())
    }
}
