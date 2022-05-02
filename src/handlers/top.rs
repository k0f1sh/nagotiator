use crate::schema::{base::AppResponse, top::NagiosStatus};
use anyhow::{anyhow, Result};
use axum::extract::Extension;
use std::sync::Arc;

use crate::state::State;

use super::base::result_to_app_response_and_logging;

async fn handle(state: Extension<Arc<State>>) -> Result<NagiosStatus> {
    let m = state.cached_state.lock().unwrap();
    let cached_state = m.as_ref().ok_or(anyhow!("not cached"))?;

    let info = cached_state.nagios_status.get_info().clone();
    let program = cached_state.nagios_status.get_program().clone();

    Ok(NagiosStatus { info, program })
}

pub async fn handler(extension: Extension<Arc<State>>) -> AppResponse<NagiosStatus> {
    result_to_app_response_and_logging(handle(extension).await)
}
