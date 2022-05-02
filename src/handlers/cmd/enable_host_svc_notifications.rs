use anyhow::{anyhow, Result};
use axum::extract::{Extension, Path};
use nagrs;
use std::sync::Arc;

use crate::{
    handlers::base::result_to_app_response_and_logging, schema::base::AppResponse, state::State,
};

use super::check::check_host_exists;

pub async fn handle(
    Path(host_name): Path<String>,
    Extension(state): Extension<Arc<State>>,
) -> Result<()> {
    {
        let m = state.cached_state.lock().unwrap();
        let cached_state = m.as_ref().ok_or(anyhow!("not cached"))?;

        check_host_exists(&cached_state.nagios_status, host_name.as_str())?;

        let cmd = nagrs::nagios::cmd::EnableHostSvcNotifications { host_name };
        state.nagrs.write_cmds(&vec![Box::new(cmd)])?;
    }

    Ok(())
}

pub async fn handler(host_name: Path<String>, extension: Extension<Arc<State>>) -> AppResponse<()> {
    result_to_app_response_and_logging(handle(host_name, extension).await)
}
