use crate::schema::{base::AppResponse, hosts::Hosts};
use axum::extract::{Extension, Path};
use regex::Regex;
use std::sync::Arc;

use crate::state::State;

pub async fn handler(
    Path(host_name_regex): Path<String>,
    Extension(state): Extension<Arc<State>>,
) -> AppResponse<Hosts> {
    let re = Regex::new(&host_name_regex);
    if re.is_err() {
        println!("hosts handler error: {:#?}", re.err().unwrap());
        return AppResponse::bad_request("invalid regex".to_string());
    }

    let hosts: Hosts;
    {
        let mut nagrs = state.nagrs.lock().unwrap();
        hosts = match nagrs.find_hosts_regex(&re.unwrap()) {
            Ok(hosts) => hosts.into_iter().map(|host| host.into()).collect(),
            Err(err) => {
                println!("hosts handler error: {:#?}", err);
                return AppResponse::internal_server_error(
                    "nagios status loading error".to_string(),
                );
            }
        };
    }

    AppResponse::success(hosts)
}
