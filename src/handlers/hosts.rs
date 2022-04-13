use axum::{
    extract::{Extension, Path},
    Json,
};
use regex::Regex;
use std::sync::Arc;

use crate::state::State;
use nagrs::nagios::Host;

pub async fn handler(
    Path(host_name_regex): Path<String>,
    Extension(state): Extension<Arc<State>>,
) -> Json<HostsResponse> {
    let re = Regex::new(&host_name_regex);
    if re.is_err() {
        // TODO logging
        return Json(HostsResponse::Error("invalid regex".to_string()));
    }

    let hosts: Vec<HostResponse>;
    {
        let mut nagrs = state.nagrs.lock().unwrap();
        hosts = match nagrs.find_hosts_regex(&re.unwrap()) {
            Ok(hosts) => hosts.into_iter().map(|host| host.into()).collect(),
            Err(_) => {
                // TODO logging
                return Json(HostsResponse::Error("nagrs error".to_string()));
            }
        };
    }

    Json(HostsResponse::Result(hosts))
}

#[derive(serde::Serialize)]
pub enum HostsResponse {
    Result(Vec<HostResponse>),
    Error(String),
}

#[derive(serde::Serialize)]
pub struct HostResponse {
    host_name: String,
    notifications_enabled: bool,
    active_checks_enabled: bool,
}

impl From<Host> for HostResponse {
    fn from(input: Host) -> Self {
        HostResponse {
            host_name: input.host_name,
            notifications_enabled: input.notifications_enabled,
            active_checks_enabled: input.active_checks_enabled,
        }
    }
}
