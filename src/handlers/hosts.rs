use crate::schema::hosts::HostResponse;
use crate::schema::hosts::Response;
use axum::{
    extract::{Extension, Path},
    Json,
};
use regex::Regex;
use std::sync::Arc;

use crate::state::State;

pub async fn handler(
    Path(host_name_regex): Path<String>,
    Extension(state): Extension<Arc<State>>,
) -> Json<Response> {
    let re = Regex::new(&host_name_regex);
    if re.is_err() {
        // TODO logging
        return Json(Response::Error("invalid regex".to_string()));
    }

    let hosts: Vec<HostResponse>;
    {
        let mut nagrs = state.nagrs.lock().unwrap();
        hosts = match nagrs.find_hosts_regex(&re.unwrap()) {
            Ok(hosts) => hosts.into_iter().map(|host| host.into()).collect(),
            Err(_) => {
                // TODO logging
                return Json(Response::Error("nagrs error".to_string()));
            }
        };
    }

    Json(Response::Result(hosts))
}
