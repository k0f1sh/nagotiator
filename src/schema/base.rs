use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use chrono::{DateTime, Utc};
use serde::Serialize;
use thiserror::Error;

#[derive(Debug)]
pub struct AppResponse<S: Serialize>(pub Result<S, AppError>);

impl<S: Serialize> IntoResponse for AppResponse<S> {
    fn into_response(self) -> Response {
        match self.0 {
            Ok(s) => (StatusCode::OK, Json(s)).into_response(),
            Err(app_error) => app_error.into_response(),
        }
    }
}

#[derive(Error, Debug, Clone)]
pub enum AppError {
    #[error("bad request: {0}")]
    BadRequest(String),
    #[error("internal server error: {0}")]
    InternalServerError(String),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        match self {
            AppError::BadRequest(s) => (StatusCode::BAD_REQUEST, s).into_response(),
            AppError::InternalServerError(s) => {
                (StatusCode::INTERNAL_SERVER_ERROR, s).into_response()
            }
        }
    }
}

#[derive(Debug, Serialize)]
pub struct WithCachedAt<S: Serialize> {
    pub cached_at: DateTime<Utc>,
    pub result: S,
}
