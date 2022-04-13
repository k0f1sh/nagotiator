use axum::{
    extract::{Extension, Path},
    Json,
};
use regex::Regex;
use std::sync::Arc;

use crate::state::State;
use nagrs::nagios::{NagiosError, Service};

pub async fn handler(
    Path(host_name_regex): Path<String>,
    Extension(state): Extension<Arc<State>>,
) -> Json<ServicesResponse> {
    let re = Regex::new(&host_name_regex);
    if re.is_err() {
        // TODO logging
        return Json(ServicesResponse::Error("invalid regex".to_string()));
    }

    let services: Vec<ServiceResponse>;
    {
        let mut nagrs = state.nagrs.lock().unwrap();
        match nagrs.find_hosts_regex(&re.unwrap()) {
            Err(_) => {
                // TODO logging
                return Json(ServicesResponse::Error("nagrs error".to_string()));
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
                        return Json(ServicesResponse::Error("nagrs error".to_string()));
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

    Json(ServicesResponse::Result(services))
}

#[derive(serde::Serialize)]
pub enum ServicesResponse {
    Result(Vec<ServiceResponse>),
    Error(String),
}

#[derive(serde::Serialize)]
pub struct ServiceResponse {
    host_name: String,
    service_description: String,
    notifications_enabled: bool,
    active_checks_enabled: bool,
    passive_checks_enabled: bool,
    check_command: String,
}

impl From<Service> for ServiceResponse {
    fn from(input: Service) -> Self {
        ServiceResponse {
            host_name: input.host_name,
            service_description: input.service_description,
            notifications_enabled: input.notifications_enabled,
            active_checks_enabled: input.active_checks_enabled,
            passive_checks_enabled: input.passive_checks_enabled,
            check_command: input.check_command,
        }
    }
}
