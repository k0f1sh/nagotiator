use nagrs::nagios::Host;

#[derive(serde::Serialize)]
pub enum Response {
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
