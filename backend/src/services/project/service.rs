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

    fn update_compose(&self, project: &ProjectInfo, compose: String) -> super::Result<String> {
        todo!()
    }

    fn env(&self, project: &ProjectInfo) -> super::Result<Option<String>> {
        todo!()
    }

    fn update_env(&self, project: &ProjectInfo, env: String) -> super::Result<String> {
        todo!()
    }
}
