use anyhow::{anyhow, Result};
use axum::extract::{Extension, Path};
use regex::Regex;
use std::sync::Arc;

use crate::{
    schema::{
        base::{AppResponse, WithCachedAt},
        services::Services,
    },
    state::State,
};

use super::base::result_to_app_response_and_logging;

pub async fn handle(
    Path(host_name_regex): Path<String>,
    Extension(state): Extension<Arc<State>>,
) -> Result<WithCachedAt<Services>> {
    let re = Regex::new(&host_name_regex)?;
    let services: Services;
    let cached_at;
    {
        let m = state.cached_state.lock().unwrap();
        let cached_state = m.as_ref().ok_or(anyhow!("not cached"))?;

        services = cached_state
            .nagios_status
            .get_hosts_regex(&re)
            .into_iter()
            .map(|host| host.host_name)
            .map(|host_name| {
                let s = cached_state
                    .nagios_status
                    .get_host_services(host_name.as_str())
                    .unwrap_or(vec![]);
                (host_name, s)
            })
            .collect::<Services>();

        cached_at = cached_state.cached_at.clone();
    }

    Ok(WithCachedAt {
        cached_at,
        result: services,
    })
}

pub async fn handler(
    host_name_regex: Path<String>,
    extension: Extension<Arc<State>>,
) -> AppResponse<WithCachedAt<Services>> {
    result_to_app_response_and_logging(handle(host_name_regex, extension).await)
}
