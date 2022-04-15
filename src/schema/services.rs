use nagrs::nagios::Service as NagrsService;

pub type Services = Vec<Service>;

#[derive(serde::Serialize)]
pub struct Service {
    host_name: String,
    service_description: String,
    notifications_enabled: bool,
    active_checks_enabled: bool,
    passive_checks_enabled: bool,
    check_command: String,
}

impl From<NagrsService> for Service {
    fn from(input: NagrsService) -> Self {
        Service {
            host_name: input.host_name,
            service_description: input.service_description,
            notifications_enabled: input.notifications_enabled,
            active_checks_enabled: input.active_checks_enabled,
            passive_checks_enabled: input.passive_checks_enabled,
            check_command: input.check_command,
        }
    }
}
