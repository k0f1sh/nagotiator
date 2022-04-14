use std::collections::HashMap;

#[derive(serde::Serialize)]
pub enum Response {
    Result(HashMap<String, String>),
    Error(String),
}
