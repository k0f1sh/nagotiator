use anyhow::Result;
use nagrs::nagios::NagiosStatus;

use crate::schema::base::AppError;

// check host exists. if host does not exists, return Err
pub fn check_host_exists(nagios_status: &NagiosStatus, host_name: &str) -> Result<()> {
    let host = nagios_status.get_host(host_name);
    match host {
        Some(_) => Ok(()),
        None => Err(AppError::BadRequest("host not found".to_string()).into()),
    }
}

// check service exists. if service does not exists, return Err
pub fn check_service_exists(
    nagios_status: &NagiosStatus,
    host_name: &str,
    service_description: &str,
) -> Result<()> {
    let services = nagios_status.get_host_services(host_name).unwrap_or(vec![]);
    let found = services
        .iter()
        .find(|service| service.service_description == service_description);

    match found {
        Some(_) => Ok(()),
        None => Err(AppError::BadRequest("service_description not found".to_string()).into()),
    }
}
