use std::sync::Arc;

use axum::{
    Router,
    extract::FromRef,
    response::{IntoResponse, Response},
};
use jsonwebtoken::{DecodingKey, EncodingKey};
use services::{
    container::{ContainerServiceError, ContainerServiceTrait},
    project::{ProjectServiceError, ProjectServiceTrait},
};
use thiserror::Error;
use tower::ServiceBuilder;
use tower_http::{
    cors::{Any, CorsLayer},
    trace::{self, TraceLayer},
};
use tracing::{Level, error};

pub mod routes;
pub mod services;

#[derive(Error, Debug)]
pub enum AppError {
    #[error(transparent)]
    Project(#[from] ProjectServiceError),

    #[error(transparent)]
    Container(#[from] ContainerServiceError),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        match self {
            AppError::Project(error) => error.into_response(),
            AppError::Container(error) => error.into_response(),
        }
    }
}

pub struct Keys {
    pub encoding: EncodingKey,
    pub decoding: DecodingKey,
}

impl Keys {
    pub fn new(secret: &[u8]) -> Self {
        Self {
            encoding: EncodingKey::from_secret(secret),
            decoding: DecodingKey::from_secret(secret),
        }
    }
}

#[derive(Clone)]
pub struct AppState {
    project_service: Arc<dyn ProjectServiceTrait>,
    container_service: Arc<dyn ContainerServiceTrait>,
    jwt_keys: Arc<Keys>,
}

impl FromRef<AppState> for Arc<dyn ProjectServiceTrait> {
    fn from_ref(input: &AppState) -> Self {
        input.project_service.clone()
    }
}

impl FromRef<AppState> for Arc<dyn ContainerServiceTrait> {
    fn from_ref(input: &AppState) -> Self {
        input.container_service.clone()
    }
}

impl FromRef<AppState> for Arc<Keys> {
    fn from_ref(input: &AppState) -> Self {
        input.jwt_keys.clone()
    }
}

pub fn app(
    project_service: Arc<dyn ProjectServiceTrait>,
    container_service: Arc<dyn ContainerServiceTrait>,
    jwt_keys: Keys,
) -> Router {
    let cors_layer = CorsLayer::new()
        .allow_headers(Any)
        .allow_origin(Any)
        .allow_methods(Any);

    let state = AppState {
        project_service,
        container_service,
        jwt_keys: Arc::new(jwt_keys),
    };

    Router::new()
        .merge(routes::routes(state))
        .layer(ServiceBuilder::new().layer(cors_layer))
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(trace::DefaultMakeSpan::new().level(Level::INFO))
                .on_response(trace::DefaultOnResponse::new().level(Level::INFO)),
        )
}
