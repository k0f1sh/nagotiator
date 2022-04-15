use nagrs::nagios::Host as NagrsHost;

pub type Hosts = Vec<Host>;

#[derive(serde::Serialize)]
pub struct Host {
    host_name: String,
    notifications_enabled: bool,
    active_checks_enabled: bool,
}

impl From<NagrsHost> for Host {
    fn from(input: NagrsHost) -> Self {
        Host {
            host_name: input.host_name,
            notifications_enabled: input.notifications_enabled,
            active_checks_enabled: input.active_checks_enabled,
        }
    }
}
