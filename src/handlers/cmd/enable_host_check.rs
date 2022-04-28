use anyhow::Result;
use axum::extract::{Extension, Path};
use nagrs;
use std::sync::Arc;

use crate::{
    handlers::base::result_to_app_response_and_logging,
    schema::base::{AppError, AppResponse},
    state::State,
};

pub async fn handle(
    Path(host_name): Path<String>,
    Extension(state): Extension<Arc<State>>,
) -> Result<()> {
    {
        let mut nagrs = state.nagrs.lock().unwrap();
        let host = nagrs.find_host(host_name.as_str())?;
        if host.is_none() {
            return Err(AppError::BadRequest("host not found".to_string()).into());
        }

        let host = host.unwrap();
        let cmd = nagrs::nagios::cmd::EnableHostCheck {
            host_name: host.host_name.to_string(),
        };
        nagrs.write_cmds(&vec![Box::new(cmd)])?;
    }

    Ok(())
}

pub async fn handler(host_name: Path<String>, extension: Extension<Arc<State>>) -> AppResponse<()> {
    result_to_app_response_and_logging(handle(host_name, extension).await)
}
