use crate::schema::base::AppResponse;
use axum::extract::{Extension, Path};
use nagrs;
use regex::Regex;
use std::sync::Arc;

use crate::state::State;

pub async fn handler(
    Path(host_name): Path<String>,
    Extension(state): Extension<Arc<State>>,
) -> AppResponse<()> {
    // exact match
    let regex_str = format!("^{}$", regex::escape(host_name.as_str()));
    let re = Regex::new(&regex_str);
    if re.is_err() {
        println!(
            "enable_host_notifications handler error: {:#?}",
            re.err().unwrap()
        );
        return AppResponse::bad_request("invalid host_name".to_string());
    }

    {
        let mut nagrs = state.nagrs.lock().unwrap();
        match nagrs.find_hosts_regex(&re.unwrap()) {
            Err(err) => {
                println!("enable_host_notifications handler error: {:#?}", err);
                return AppResponse::internal_server_error(
                    "nagios status loading error".to_string(),
                );
            }
            Ok(hosts) => {
                if hosts.len() != 1 {
                    return AppResponse::bad_request(format!(
                        "host \"{}\" does not exist",
                        host_name.to_string()
                    ));
                }

                let host = &hosts[0];
                let cmd =
                    nagrs::nagios::cmd::EnableHostNotifications::new(host.host_name.to_string());

                let result = nagrs.write_cmds(&vec![Box::new(cmd)]);
                match result {
                    Ok(_) => {
                        return AppResponse::success(());
                    }
                    Err(err) => {
                        println!("enable_host_notifications handler error: {:#?}", err);
                        return AppResponse::internal_server_error(
                            "faild to write commands".to_string(),
                        );
                    }
                }
            }
        };
    }
}
