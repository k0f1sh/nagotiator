use crate::schema::{base::AppResponse, top::NagiosInfo};
use axum::extract::Extension;
use std::sync::Arc;

use crate::state::State;

pub async fn handler(Extension(state): Extension<Arc<State>>) -> AppResponse<NagiosInfo> {
    {
        let mut nagrs = state.nagrs.lock().unwrap();
        match nagrs.get_info() {
            Err(_) => AppResponse::internal_server_error("error".to_string()),
            Ok(info) => AppResponse::success(info),
        }
    }
}
