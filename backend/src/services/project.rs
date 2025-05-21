use super::{ProjectInfo, ProjectServiceTrait};

#[derive(Default)]
pub struct ProjectService;

impl ProjectServiceTrait for ProjectService {
    fn all_projects(&self) -> Result<Vec<ProjectInfo>, String> {
        todo!()
    }

    fn compose(&self, project: &ProjectInfo) -> Result<String, String> {
        todo!()
    }

    fn env(&self, project: &ProjectInfo) -> Result<String, String> {
        todo!()
    }
}
