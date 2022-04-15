use crate::schema::{base::AppResponse, top::NagiosInfo};
use axum::{extract::Extension, http::StatusCode};
use std::sync::Arc;

use crate::state::State;

pub async fn handler(Extension(state): Extension<Arc<State>>) -> AppResponse<NagiosInfo> {
    {
        let mut nagrs = state.nagrs.lock().unwrap();
        match nagrs.get_info() {
            Err(_) => AppResponse::error(StatusCode::INTERNAL_SERVER_ERROR, "error".to_string()),
            Ok(info) => AppResponse::success(info),
        }
    }
}
