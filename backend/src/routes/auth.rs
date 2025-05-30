use std::sync::Arc;
use std::time::SystemTime;

use axum::RequestPartsExt;
use axum::extract::{FromRequestParts, State};
use axum::http::request::Parts;
use axum::response::Response;
use axum::routing::{get, post};
use axum::{Json, Router, http::StatusCode, response::IntoResponse};

use axum_extra::TypedHeader;
use axum_extra::headers::authorization::Bearer;
use axum_extra::headers::{Authorization, Cookie};
use jsonwebtoken::{Header, Validation, decode, encode};
use serde::{Deserialize, Serialize};
use serde_json::json;
use thiserror::Error;

use crate::{AdminAuth, AppState, Keys};

#[derive(Error, Debug)]
pub enum AuthError {
    #[error("Wrong credentials")]
    WrongCredentials,

    #[error("Missing credentials")]
    MissingCredentials,

    #[error("Token creation error")]
    TokenCreation,

    #[error("Invalid token")]
    InvalidToken,
}

impl IntoResponse for AuthError {
    fn into_response(self) -> Response {
        let status = match self {
            AuthError::WrongCredentials => StatusCode::UNAUTHORIZED,
            AuthError::MissingCredentials => StatusCode::BAD_REQUEST,
            AuthError::TokenCreation => StatusCode::INTERNAL_SERVER_ERROR,
            AuthError::InvalidToken => StatusCode::UNAUTHORIZED,
        };
        let body = Json(json!({ "error": self.to_string() }));
        (status, body).into_response()
    }
}

pub fn routes(state: AppState) -> Router {
    Router::new()
        .route("/", post(authorize))
        .route("/validate", get(validate))
        .with_state(state)
}

async fn validate(_claims: Claims) -> Result<(), AuthError> {
    Ok(())
}

#[derive(Debug, Deserialize)]
struct AuthPayload {
    user: String,
    pw: String,
}

async fn authorize(
    State(keys): State<Arc<Keys>>,
    State(admin_auth): State<Arc<AdminAuth>>,
    Json(payload): Json<AuthPayload>,
) -> Result<impl IntoResponse, AuthError> {
    if payload.user.is_empty() || payload.pw.is_empty() {
        return Err(AuthError::MissingCredentials);
    }

    if payload.user != admin_auth.name || payload.pw != admin_auth.password {
        return Err(AuthError::WrongCredentials);
    }

    let now = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_secs();

    let claims = Claims {
        sub: "admin".to_owned(),
        iat: now,
        exp: now + 60 * 60 * 24 * 30, // 1 month,
    };

    let token = encode(&Header::default(), &claims, &keys.encoding)
        .map_err(|_| AuthError::TokenCreation)?;

    Ok(Json(json!({ "token": token})))
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    sub: String,
    iat: u64,
    exp: u64,
}

impl FromRequestParts<AppState> for Claims {
    type Rejection = AuthError;

    async fn from_request_parts(
        parts: &mut Parts,
        keys: &AppState,
    ) -> Result<Self, Self::Rejection> {
        // Extract the token from the authorization header
        let token = if let Ok(TypedHeader(Authorization(bearer))) =
            parts.extract::<TypedHeader<Authorization<Bearer>>>().await
        {
            bearer.token().to_string()
        } else {
            // Fallback to cookie named "token"
            let cookies = parts
                .extract::<TypedHeader<Cookie>>()
                .await
                .map_err(|_| AuthError::InvalidToken)?;
            cookies
                .get("token")
                .ok_or(AuthError::InvalidToken)?
                .to_string()
        };

        // Decode the user data
        let token_data = decode::<Claims>(
            &token,
            &keys.jwt_keys.decoding.clone(),
            &Validation::default(),
        )
        .map_err(|_err| AuthError::InvalidToken)?;

        Ok(token_data.claims)
    }
}
