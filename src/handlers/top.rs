use axum::{extract::Extension, Json};
use std::sync::Arc;

use crate::state::State;

pub async fn handler(Extension(_state): Extension<Arc<State>>) -> Json<TopResponse> {
    Json(TopResponse {
        status: "ok".to_string(),
    })
}

#[derive(serde::Serialize)]
pub struct TopResponse {
    status: String,
}
