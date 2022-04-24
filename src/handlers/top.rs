use crate::schema::{base::AppResponse, top::NagiosStatus};
use axum::extract::Extension;
use std::sync::Arc;

use crate::state::State;

pub async fn handler(Extension(state): Extension<Arc<State>>) -> AppResponse<NagiosStatus> {
    {
        let mut nagrs = state.nagrs.lock().unwrap();

        let info = nagrs.get_info();
        if info.is_err() {
            println!("top handler error: {:#?}", info.err().unwrap());
            return AppResponse::internal_server_error("error".to_string());
        }

        let program = nagrs.get_program();
        if program.is_err() {
            println!("top handler error: {:#?}", program.err().unwrap());
            return AppResponse::internal_server_error("error".to_string());
        }

        AppResponse::success(NagiosStatus {
            info: info.unwrap(),
            program: program.unwrap(),
        })
    }
}
