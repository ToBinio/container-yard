use std::sync::Arc;

use axum::{
    Json, Router,
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::get,
};
use services::ProjectServiceTrait;
use tower_http::trace::{self, TraceLayer};
use tracing::{Level, error};

pub mod services;

pub fn app<PROJECT: ProjectServiceTrait + 'static>(project_service: Arc<PROJECT>) -> Router {
    Router::new()
        .route("/containers", get(get_all_containers))
        .with_state(project_service)
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(trace::DefaultMakeSpan::new().level(Level::INFO))
                .on_response(trace::DefaultOnResponse::new().level(Level::INFO)),
        )
}

async fn get_all_containers<PROJECT: ProjectServiceTrait>(
    State(project_service): State<Arc<PROJECT>>,
) -> Response {
    let projects = project_service.all_projects();

    match projects {
        Ok(ok) => Json(ok).into_response(),
        Err(err) => {
            error!("Error getting container stats: {}", err);
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}
