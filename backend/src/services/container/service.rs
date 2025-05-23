use super::{ContainerServiceTrait, ProjectInfo};

#[derive(Default)]
pub struct ContainerService;

impl ContainerServiceTrait for ContainerService {
    fn are_online(&self, _projects: &Vec<ProjectInfo>) -> super::Result<Vec<bool>> {
        todo!()
    }

    fn is_online(&self, _project: &ProjectInfo) -> super::Result<bool> {
        todo!()
    }

    fn stop(&self, _project: &ProjectInfo) -> super::Result<()> {
        todo!()
    }

    fn start(&self, _project: &ProjectInfo) -> super::Result<()> {
        todo!()
    }

    fn update(&self, _project: &ProjectInfo) -> super::Result<()> {
        todo!()
    }
}
