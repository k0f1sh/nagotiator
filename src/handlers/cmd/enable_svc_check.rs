use anyhow::Result;
use axum::extract::{Extension, Path};
use nagrs;
use std::sync::Arc;

use crate::{
    handlers::base::result_to_app_response_and_logging,
    schema::base::{AppError, AppResponse},
    state::State,
};

use super::check::check_host_exists;

pub async fn handle(
    Path(host_name): Path<String>,
    Path(service_description): Path<String>,
    Extension(state): Extension<Arc<State>>,
) -> Result<()> {
    {
        let mut nagrs = state.nagrs.lock().unwrap();

        check_host_exists(&mut nagrs, host_name.as_str())?;

        let services = nagrs.find_services(host_name.as_str())?;
        let found = services
            .iter()
            .find(|service| service.service_description == service_description);
        if found.is_none() {
            return Err(AppError::BadRequest("service_description not found".to_string()).into());
        }

        let cmd = nagrs::nagios::cmd::EnableSvcCheck {
            host_name,
            service_description,
        };
        nagrs.write_cmds(&vec![Box::new(cmd)])?;
    }

    Ok(())
}

pub async fn handler(
    host_name: Path<String>,
    service_description: Path<String>,
    extension: Extension<Arc<State>>,
) -> AppResponse<()> {
    result_to_app_response_and_logging(handle(host_name, service_description, extension).await)
}
