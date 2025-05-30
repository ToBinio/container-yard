use axum::Router;

use crate::AppState;

pub mod auth;
pub mod projects;

pub fn routes(state: AppState) -> Router {
    Router::new()
        .nest("/auth", auth::routes(state.clone()))
        .nest("/projects", projects::routes(state.clone()))
}
