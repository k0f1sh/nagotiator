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
