use anyhow::Result;
use axum::extract::{Extension, Path};
use nagrs;
use regex::Regex;
use std::sync::Arc;

use crate::{
    handlers::base::result_to_app_apesponse_and_logging,
    schema::base::{AppError, AppResponse},
    state::State,
};

pub async fn handle(
    Path(host_name): Path<String>,
    Extension(state): Extension<Arc<State>>,
) -> Result<()> {
    // exact match
    let regex_str = format!("^{}$", regex::escape(host_name.as_str()));
    let re = Regex::new(&regex_str)?;

    {
        let mut nagrs = state.nagrs.lock().unwrap();
        let hosts = nagrs.find_hosts_regex(&re)?;
        if hosts.len() != 1 {
            return Err(AppError::BadRequest("host not found".to_string()).into());
        }

        let host = &hosts[0];
        let cmd = nagrs::nagios::cmd::EnableHostNotifications {
            host_name: host.host_name.to_string(),
        };
        nagrs.write_cmds(&vec![Box::new(cmd)])?;
    }

    Ok(())
}

pub async fn handler(host_name: Path<String>, extension: Extension<Arc<State>>) -> AppResponse<()> {
    result_to_app_apesponse_and_logging(handle(host_name, extension).await)
}
