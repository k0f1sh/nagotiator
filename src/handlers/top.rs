use crate::schema::top::Response;
use axum::{extract::Extension, Json};
use std::sync::Arc;

use crate::state::State;

pub async fn handler(Extension(state): Extension<Arc<State>>) -> Json<Response> {
    {
        let mut nagrs = state.nagrs.lock().unwrap();
        match nagrs.get_info() {
            Err(_) => Json(Response::Error("error".to_string())),
            Ok(info) => Json(Response::Result(info)),
        }
    }
}
