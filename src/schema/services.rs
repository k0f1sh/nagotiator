use nagrs::nagios::Service as NagrsService;

pub type Services = Vec<Service>;

#[derive(serde::Serialize)]
pub struct Service {
    host_name: String,
    check_command: String,
    service_description: String,
    active_checks_enabled: bool,
    passive_checks_enabled: bool,
    obsess: bool,
    event_handler_enabled: bool,
    flap_detection_enabled: bool,
    notifications_enabled: bool,
}

impl From<NagrsService> for Service {
    fn from(input: NagrsService) -> Self {
        Service {
            host_name: input.host_name,
            check_command: input.check_command,
            service_description: input.service_description,
            active_checks_enabled: input.active_checks_enabled,
            passive_checks_enabled: input.passive_checks_enabled,
            obsess: input.obsess,
            event_handler_enabled: input.event_handler_enabled,
            flap_detection_enabled: input.flap_detection_enabled,
            notifications_enabled: input.notifications_enabled,
        }
    }
}
