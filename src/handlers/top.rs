use axum::{extract::Extension, Json};
use std::sync::Arc;

use crate::state::State;

pub async fn handler(Extension(_state): Extension<Arc<State>>) -> Json<TopResponse> {
    // let host: Option<Host>;
    // {
    //     let mut nagrs = state.nagrs.lock().unwrap();
    //     host = match nagrs.find_host("localhost") {
    //         Ok(host) => host,
    //         Err(_) => return "nagrs error".to_string(),
    //     };
    // }
    Json(TopResponse {
        status: "ok".to_string(),
    })
}

#[derive(serde::Serialize)]
pub struct TopResponse {
    status: String,
}
