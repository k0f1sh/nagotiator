use crate::schema::{base::AppResponse, hosts::Hosts};
use anyhow::Result;
use axum::extract::{Extension, Path};
use regex::Regex;
use std::sync::Arc;

use crate::state::State;

use super::base::result_to_app_apesponse_and_logging;

pub async fn handle(
    Path(host_name_regex): Path<String>,
    Extension(state): Extension<Arc<State>>,
) -> Result<Hosts> {
    let re = Regex::new(&host_name_regex)?;

    let hosts: Hosts;
    {
        let mut nagrs = state.nagrs.lock().unwrap();
        hosts = nagrs
            .find_hosts_regex(&re)?
            .into_iter()
            .map(|host| host.into())
            .collect();
    }

    Ok(hosts)
}

pub async fn handler(
    host_name_regex: Path<String>,
    extension: Extension<Arc<State>>,
) -> AppResponse<Hosts> {
    result_to_app_apesponse_and_logging(handle(host_name_regex, extension).await)
}
