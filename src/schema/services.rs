use nagrs::nagios::Service;

#[derive(serde::Serialize)]
pub enum Response {
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
