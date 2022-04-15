use crate::schema::{base::AppResponse, hosts::Hosts};
use axum::{
    extract::{Extension, Path},
    http::StatusCode,
};
use regex::Regex;
use std::sync::Arc;

use crate::state::State;

pub async fn handler(
    Path(host_name_regex): Path<String>,
    Extension(state): Extension<Arc<State>>,
) -> AppResponse<Hosts> {
    let re = Regex::new(&host_name_regex);
    if re.is_err() {
        // TODO logging
        return AppResponse::error(StatusCode::BAD_REQUEST, "invalid regex".to_string());
    }

    let hosts: Hosts;
    {
        let mut nagrs = state.nagrs.lock().unwrap();
        hosts = match nagrs.find_hosts_regex(&re.unwrap()) {
            Ok(hosts) => hosts.into_iter().map(|host| host.into()).collect(),
            Err(_) => {
                // TODO logging
                return AppResponse::error(
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "nagios status loading error".to_string(),
                );
            }
        };
    }

    AppResponse::success(hosts)
}
