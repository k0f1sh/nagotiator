use anyhow::Result;
use serde::Serialize;

use crate::schema::base::{AppError, AppResponse};

pub fn result_to_app_apesponse_and_logging<S: Serialize>(s: Result<S>) -> AppResponse<S> {
    match s {
        Ok(s) => AppResponse(Ok(s)),
        Err(error) => {
            println!("{:#?}", error);

            if let Some(app_error) = error.downcast_ref::<AppError>() {
                return AppResponse(Err(app_error.clone()));
            }

            AppResponse(Err(AppError::InternalServerError(
                "server error".to_string(),
            )))
        }
    }
}
