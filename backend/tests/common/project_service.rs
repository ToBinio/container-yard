#![allow(dead_code)]

use std::{
    fs::{File, create_dir},
    io::Write,
    path::PathBuf,
};

use backend::services::project::service::ProjectService;
use tempfile::TempDir;

pub fn test_project_service() -> (TempDir, ProjectService) {
    let dir = TempDir::new().unwrap();
    let path: PathBuf = dir.path().into();

    let path_project_1 = path.join("project1");
    let path_project_1_sub = path_project_1.join("sub");
    let path_project_2 = path.join("project2");
    let path_project_3 = path.join("project3");

    create_dir(&path_project_1).unwrap();
    create_dir(&path_project_1_sub).unwrap();
    create_dir(&path_project_2).unwrap();
    create_dir(&path_project_3).unwrap();

    File::create(path_project_1.join("compose.yml"))
        .unwrap()
        .write_all(b"compose.yml")
        .unwrap();

    File::create(path_project_1.join(".env"))
        .unwrap()
        .write_all(b".env")
        .unwrap();

    File::create(path_project_1_sub.join("text.txt"))
        .unwrap()
        .write_all(b"sub file")
        .unwrap();

    File::create(path_project_3.join("compose.yml"))
        .unwrap()
        .write_all(b"compose.yml")
        .unwrap();

    (dir, ProjectService::new(path))
}
