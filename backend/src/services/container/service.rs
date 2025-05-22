use super::{ContainerServiceTrait, ProjectInfo};

#[derive(Default)]
pub struct ContainerService;

impl ContainerServiceTrait for ContainerService {
    fn are_online(&self, projects: &Vec<ProjectInfo>) -> super::Result<Vec<bool>> {
        todo!()
    }

    fn is_online(&self, project: &ProjectInfo) -> super::Result<bool> {
        todo!()
    }

    fn stop(&self, project: &ProjectInfo) -> super::Result<bool> {
        todo!()
    }

    fn start(&self, project: &ProjectInfo) -> super::Result<bool> {
        todo!()
    }
}
