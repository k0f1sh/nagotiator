use anyhow::Result;
use axum::extract::{Extension, Path};
use regex::Regex;
use std::sync::Arc;

use crate::{
    schema::{base::AppResponse, services::Services},
    state::State,
};
use nagrs::nagios::object::Service;

use super::base::result_to_app_response_and_logging;

pub async fn handle(
    Path(host_name_regex): Path<String>,
    Extension(state): Extension<Arc<State>>,
) -> Result<Services> {
    let re = Regex::new(&host_name_regex)?;
    let services_list: Vec<Services>;
    {
        let mut nagrs = state.nagrs.lock().unwrap();
        services_list = nagrs
            .find_hosts_regex(&re)?
            .into_iter()
            .map(|host| host.host_name)
            .map(|host_name| nagrs.find_services(host_name.as_str()))
            .collect::<Result<Vec<Vec<Service>>, _>>()?;
    }

    let services = services_list
        .into_iter()
        .flatten()
        .map(|service| service.into())
        .collect();

    Ok(services)
}

pub async fn handler(
    host_name_regex: Path<String>,
    extension: Extension<Arc<State>>,
) -> AppResponse<Services> {
    result_to_app_response_and_logging(handle(host_name_regex, extension).await)
}
