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
                name: "test".to_string(),
                dir: path.join("project1"),
            },
            ProjectInfo {
                name: "test2".to_string(),
                dir: path.join("project2"),
            },
            ProjectInfo {
                name: "test3".to_string(),
                dir: path.join("project3"),
            }
        ])
    )
}

#[tokio::test]
async fn get_project() {
    let (dir, project_service) = test_project_service();
    let path = dir.path();

    let projects = project_service.project("test2");

    assert_eq!(
        projects,
        Ok(ProjectInfo {
            name: "test".to_string(),
            dir: path.join("project2"),
        })
    )
}

#[tokio::test]
async fn get_project_unknown() {
    let (_dir, project_service) = test_project_service();

    let projects = project_service.project("test404");

    assert_eq!(
        projects,
        Err(ProjectServiceError::NotFound("test404".to_string()))
    )
}

#[tokio::test]
async fn get_project_compose() {
    let (_dir, project_service) = test_project_service();

    let project_info = project_service.project("test").unwrap();
    let projects = project_service.compose(&project_info);

    assert_eq!(projects, Ok("compose.yml".to_string()))
}

#[tokio::test]
async fn get_project_compose_missing() {
    let (_dir, project_service) = test_project_service();

    let project_info = project_service.project("test2").unwrap();
    let projects = project_service.compose(&project_info);

    assert_eq!(
        projects,
        Err(ProjectServiceError::NotFound("test2".to_string()))
    );
}

#[tokio::test]
async fn get_project_env() {
    let (_dir, project_service) = test_project_service();

    let project_info = project_service.project("test").unwrap();
    let projects = project_service.env(&project_info);

    assert_eq!(projects, Ok(Some(".env".to_string())))
}

#[tokio::test]
async fn get_project_env_missing() {
    let (_dir, project_service) = test_project_service();

    let project_info = project_service.project("test2").unwrap();
    let projects = project_service.env(&project_info);

    assert_eq!(projects, Ok(None));
}

#[tokio::test]
async fn update_project_compose() {
    let (_dir, project_service) = test_project_service();

    let project_info = project_service.project("test").unwrap();

    let projects = project_service.compose(&project_info);
    assert_eq!(projects, Ok("compose.yml".to_string()));

    let projects = project_service.update_compose(&project_info, "newCompose".to_string());
    assert_eq!(projects, Ok("newCompose".to_string()));

    let projects = project_service.compose(&project_info);
    assert_eq!(projects, Ok("newCompose".to_string()));
}

#[tokio::test]
async fn update_project_compose_missing() {
    let (_dir, project_service) = test_project_service();

    let project_info = project_service.project("test2").unwrap();

    let projects = project_service.compose(&project_info);
    assert_eq!(
        projects,
        Err(ProjectServiceError::NotFound("test2".to_string()))
    );

    let projects = project_service.update_compose(&project_info, "newCompose".to_string());
    assert_eq!(projects, Ok("newCompose".to_string()));

    let projects = project_service.compose(&project_info);
    assert_eq!(projects, Ok("newCompose".to_string()));
}

#[tokio::test]
async fn update_project_env() {
    let (_dir, project_service) = test_project_service();

    let project_info = project_service.project("test").unwrap();

    let projects = project_service.env(&project_info);
    assert_eq!(projects, Ok(Some(".env".to_string())));

    let projects = project_service.update_env(&project_info, "new env".to_string());
    assert_eq!(projects, Ok("new env".to_string()));

    let projects = project_service.env(&project_info);
    assert_eq!(projects, Ok(Some("new env".to_string())));
}

#[tokio::test]
async fn update_project_env_missing() {
    let (_dir, project_service) = test_project_service();

    let project_info = project_service.project("test2").unwrap();

    let projects = project_service.env(&project_info);
    assert_eq!(projects, Ok(None));

    let projects = project_service.update_env(&project_info, "new env".to_string());
    assert_eq!(projects, Ok("new env".to_string()));

    let projects = project_service.env(&project_info);
    assert_eq!(projects, Ok(Some("new env".to_string())));
}
