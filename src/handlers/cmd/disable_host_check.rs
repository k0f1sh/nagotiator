use anyhow::Result;
use axum::extract::{Extension, Path};
use nagrs;
use std::sync::Arc;

use crate::{
    handlers::base::result_to_app_response_and_logging, schema::base::AppResponse, state::State,
};

use super::check::check_host_exists;

async fn handle(
    Path(host_name): Path<String>,
    Extension(state): Extension<Arc<State>>,
) -> Result<()> {
    {
        let mut nagrs = state.nagrs.lock().unwrap();
        check_host_exists(&mut nagrs, host_name.as_str())?;

        let cmd = nagrs::nagios::cmd::DisableHostCheck { host_name };
        nagrs.write_cmds(&vec![Box::new(cmd)])?;
    }

    Ok(())
}

pub async fn handler(host_name: Path<String>, extension: Extension<Arc<State>>) -> AppResponse<()> {
    result_to_app_response_and_logging(handle(host_name, extension).await)
}
