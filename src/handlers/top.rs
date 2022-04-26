use crate::schema::{base::AppResponse, top::NagiosStatus};
use anyhow::Result;
use axum::extract::Extension;
use std::sync::Arc;

use crate::state::State;

use super::base::result_to_app_apesponse_and_logging;

async fn handle(state: Extension<Arc<State>>) -> Result<NagiosStatus> {
    let mut nagrs = state.nagrs.lock().unwrap();

    let info = nagrs.get_info()?;
    let program = nagrs.get_program()?;

    Ok(NagiosStatus { info, program })
}

pub async fn handler(extension: Extension<Arc<State>>) -> AppResponse<NagiosStatus> {
    result_to_app_apesponse_and_logging(handle(extension).await)
}
