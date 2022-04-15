use axum::{
    extract::{Extension, Path},
    http::StatusCode,
};
use regex::Regex;
use std::sync::Arc;

use crate::{
    schema::{base::AppResponse, services::Services},
    state::State,
};
use nagrs::nagios::{NagiosError, Service};

pub async fn handler(
    Path(host_name_regex): Path<String>,
    Extension(state): Extension<Arc<State>>,
) -> AppResponse<Services> {
    let re = Regex::new(&host_name_regex);
    if re.is_err() {
        // TODO logging
        return AppResponse::error(StatusCode::BAD_REQUEST, "invalid regex".to_string());
    }

    let services: Services;
    {
        let mut nagrs = state.nagrs.lock().unwrap();
        match nagrs.find_hosts_regex(&re.unwrap()) {
            Err(_) => {
                // TODO logging
                return AppResponse::error(
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "nagios status loading error".to_string(),
                );
            }
            Ok(hosts) => {
                let services_list = hosts
                    .into_iter()
                    .map(|host| host.host_name)
                    .map(|host_name| nagrs.find_services(host_name.as_str()))
                    .collect::<Result<Vec<Vec<Service>>, NagiosError>>();

                match services_list {
                    Err(_) => {
                        // TODO logging
                        return AppResponse::error(
                            StatusCode::INTERNAL_SERVER_ERROR,
                            "nagios status loading error".to_string(),
                        );
                    }
                    Ok(services_list) => {
                        services = services_list
                            .into_iter()
                            .flatten()
                            .map(|service| service.into())
                            .collect();
                    }
                }
            }
        };
    }

    AppResponse::success(services)
}
