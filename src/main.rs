use std::sync::Arc;

use axum::{extract::Extension, routing::get, Router};
use nagotiator::{handlers, state::State};

#[tokio::main]
async fn main() {
    let state: Arc<State> = Arc::new(State::new());

    let app = Router::new()
        .route("/", get(handlers::top::handler))
        .route("/hosts/:host_name_regex", get(handlers::hosts::handler))
        .route(
            "/services/:host_name_regex",
            get(handlers::services::handler),
        )
        .layer(Extension(state));

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
