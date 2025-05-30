use axum::Router;

use crate::AppState;

pub fn routes(state: AppState) -> Router {
    Router::new().with_state(state)
}
