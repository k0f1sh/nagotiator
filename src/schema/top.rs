use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Deserialize, Serialize)]
pub struct NagiosStatus {
    pub info: HashMap<String, String>,
    pub program: HashMap<String, String>,
}
