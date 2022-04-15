use axum::Json;
use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::Serialize;

pub enum AppResponse<S>
where
    S: Serialize,
{
    Success(S),
    Error(StatusCode, String),
}

impl<S> IntoResponse for AppResponse<S>
where
    S: Serialize,
{
    fn into_response(self) -> Response {
        match self {
            Self::Success(s) => (StatusCode::OK, Json(s)).into_response(),
            Self::Error(status_code, error_message) => (status_code, error_message).into_response(),
        }
    }
}

impl<S> AppResponse<S>
where
    S: Serialize,
{
    pub fn success(s: S) -> Self {
        Self::Success(s)
    }

    pub fn error(status_code: StatusCode, error_message: String) -> Self {
        Self::Error(status_code, error_message)
    }
}
