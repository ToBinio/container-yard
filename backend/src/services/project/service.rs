use super::{ProjectInfo, ProjectServiceTrait};

#[derive(Default)]
pub struct ProjectService;

impl ProjectServiceTrait for ProjectService {
    fn all_projects(&self) -> super::Result<Vec<ProjectInfo>> {
        todo!()
    }

    fn project(&self, name: String) -> super::Result<ProjectInfo> {
        todo!()
    }

    fn compose(&self, project: &ProjectInfo) -> super::Result<String> {
        todo!()
    }

    fn env(&self, project: &ProjectInfo) -> super::Result<String> {
        todo!()
    }
}
