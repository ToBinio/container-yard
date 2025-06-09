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
        Err(ProjectServiceError::ProjectNotFound(
            "project404".to_string()
        ))
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
async fn read_project_file() {
    let (_dir, project_service) = test_project_service();

    let project_info = project_service.project("project1").unwrap();
    let content = project_service
        .read_file(&project_info, "compose.yml")
        .unwrap();

    assert_eq!(content, "compose.yml")
}

#[tokio::test]
async fn read_project_file_not_base() {
    let (_dir, project_service) = test_project_service();

    let project_info = project_service.project("project1").unwrap();
    let error = project_service.read_file(&project_info, "sub/text.txt");

    assert_eq!(
        error,
        Err(ProjectServiceError::InvalidFilePath(
            "sub/text.txt".to_string()
        ))
    );

    let error = project_service.read_file(&project_info, "../project3/compose.yml");

    assert_eq!(
        error,
        Err(ProjectServiceError::InvalidFilePath(
            "../project3/compose.yml".to_string()
        ))
    );
}

#[tokio::test]
async fn update_project_file() {
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
async fn update_project_file_non_existin() {
    let (_dir, project_service) = test_project_service();

    let project_info = project_service.project("project1").unwrap();
    let content = project_service
        .update_file(&project_info, "newFile", "newContent")
        .unwrap();
    assert_eq!(content, "newContent");

    let content = project_service.read_file(&project_info, "newFile").unwrap();
    assert_eq!(content, "newContent");
}

#[tokio::test]
async fn update_project_file_not_base() {
    let (_dir, project_service) = test_project_service();

    let project_info = project_service.project("project1").unwrap();
    let error = project_service.update_file(&project_info, "sub/text.txt", "content");

    assert_eq!(
        error,
        Err(ProjectServiceError::InvalidFilePath(
            "sub/text.txt".to_string()
        ))
    );

    let error = project_service.update_file(&project_info, "../project3/compose.yml", "content");

    assert_eq!(
        error,
        Err(ProjectServiceError::InvalidFilePath(
            "../project3/compose.yml".to_string()
        ))
    );
}

#[tokio::test]
async fn create_new_project() {
    let (dir, project_service) = test_project_service();
    let path = dir.path();

    let project_info = project_service.create("newProject");

    assert_eq!(
        project_info,
        Ok(ProjectInfo {
            name: "newProject".to_string(),
            dir: path.join("newProject"),
        })
    );

    let files = project_service.files(&project_info.unwrap());

    assert_eq!(files, Ok(vec!["compose.yml".to_string()]));
}

#[tokio::test]
async fn create_already_existing_project() {
    let (_dir, project_service) = test_project_service();

    let project_info = project_service.create("project1");

    assert_eq!(
        project_info,
        Err(ProjectServiceError::ProjectAlreadyExists(
            "project1".to_string()
        ))
    );
}

#[tokio::test]
async fn create_invalid_project() {
    let (_dir, project_service) = test_project_service();

    let error = project_service.create("test/newProject");

    assert_eq!(
        error,
        Err(ProjectServiceError::InvalidFilePath(
            "test/newProject".to_string()
        ))
    );

    let error = project_service.create("../newProject");

    assert_eq!(
        error,
        Err(ProjectServiceError::InvalidFilePath(
            "../newProject".to_string()
        ))
    );
}

#[tokio::test]
async fn delete_project_file() {
    let (_dir, project_service) = test_project_service();

    let project_info = project_service.project("project1").unwrap();
    project_service
        .delete_file(&project_info, "compose.yml")
        .unwrap();

    let err = project_service.read_file(&project_info, "compose.yml");
    assert_eq!(
        err,
        Err(ProjectServiceError::FileNotFound {
            project: "project1".to_string(),
            file: "compose.yml".to_string()
        })
    );
}

#[tokio::test]
async fn delete_project_unknown_file() {
    let (_dir, project_service) = test_project_service();

    let project_info = project_service.project("project1").unwrap();
    let err = project_service.delete_file(&project_info, "unknown.txt");

    assert_eq!(
        err,
        Err(ProjectServiceError::FileNotFound {
            project: "project1".to_string(),
            file: "unknown.txt".to_string()
        })
    );
}

#[tokio::test]
async fn delete_project() {
    let (dir, project_service) = test_project_service();
    let path = dir.path();

    let project_info = project_service.project("project1").unwrap();

    project_service.delete(&project_info).unwrap();

    let projects = project_service.all_projects();
    assert_eq!(
        projects,
        Ok(vec![
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
