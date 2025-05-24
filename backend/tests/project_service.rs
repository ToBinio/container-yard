use backend::services::project::{ProjectInfo, ProjectServiceError, ProjectServiceTrait};
use common::project_service::test_project_service;

mod common;

#[tokio::test]
async fn get_projects() {
    let (dir, project_service) = test_project_service();
    let path = dir.path();

    let projects = project_service.all_projects();

    assert_eq!(
        projects,
        Ok(vec![
            ProjectInfo {
                name: "project1".to_string(),
                dir: path.join("project1"),
            },
            ProjectInfo {
                name: "project2".to_string(),
                dir: path.join("project2"),
            },
            ProjectInfo {
                name: "project3".to_string(),
                dir: path.join("project3"),
            }
        ])
    )
}

#[tokio::test]
async fn get_project() {
    let (dir, project_service) = test_project_service();
    let path = dir.path();

    let projects = project_service.project("project2");

    assert_eq!(
        projects,
        Ok(ProjectInfo {
            name: "project2".to_string(),
            dir: path.join("project2"),
        })
    )
}

#[tokio::test]
async fn get_project_unknown() {
    let (_dir, project_service) = test_project_service();

    let projects = project_service.project("project404");

    assert_eq!(
        projects,
        Err(ProjectServiceError::NotFound("project404".to_string()))
    )
}

#[tokio::test]
async fn get_project_files() {
    let (_dir, project_service) = test_project_service();

    let project_info = project_service.project("project1").unwrap();
    let mut files = project_service.files(&project_info).unwrap();

    files.sort();

    assert_eq!(files, vec![".env".to_string(), "compose.yml".to_string()])
}

#[tokio::test]
async fn read_project_files() {
    let (_dir, project_service) = test_project_service();

    let project_info = project_service.project("project1").unwrap();
    let content = project_service
        .read_file(&project_info, "compose.yml")
        .unwrap();

    assert_eq!(content, "compose.yml")
}

#[tokio::test]
async fn update_project_files() {
    let (_dir, project_service) = test_project_service();

    let project_info = project_service.project("project1").unwrap();
    let content = project_service
        .update_file(&project_info, "compose.yml", "newCompose")
        .unwrap();
    assert_eq!(content, "newCompose");

    let content = project_service
        .read_file(&project_info, "compose.yml")
        .unwrap();
    assert_eq!(content, "newCompose");
}

#[tokio::test]
async fn update_project_files_non_existin() {
    let (_dir, project_service) = test_project_service();

    let project_info = project_service.project("project1").unwrap();
    let content = project_service
        .update_file(&project_info, "newFile", "newContent")
        .unwrap();
    assert_eq!(content, "newContent");

    let content = project_service.read_file(&project_info, "newFile").unwrap();
    assert_eq!(content, "newContent");
}
