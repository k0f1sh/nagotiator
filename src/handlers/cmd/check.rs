use std::sync::MutexGuard;

use anyhow::Result;

use crate::{schema::base::AppError, state::Nagrs};

// check host exists. if host does not exists, return Err
pub fn check_host_exists(nagrs: &mut MutexGuard<Nagrs>, host_name: &str) -> Result<()> {
    let host = nagrs.find_host(host_name)?;
    match host {
        Some(_) => Ok(()),
        None => Err(AppError::BadRequest("host not found".to_string()).into()),
    }
}

// check service exists. if service does not exists, return Err
pub fn check_service_exists(
    nagrs: &mut MutexGuard<Nagrs>,
    host_name: &str,
    service_description: &str,
) -> Result<()> {
    let services = nagrs.find_services(host_name)?;
    let found = services
        .iter()
        .find(|service| service.service_description == service_description);

    match found {
        Some(_) => Ok(()),
        None => Err(AppError::BadRequest("service_description not found".to_string()).into()),
    }
}
