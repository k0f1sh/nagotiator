use axum::routing::post;
use std::sync::Arc;

use axum::{extract::Extension, routing::get, Router};
use clap::Parser;
use nagotiator::{handlers, state::State};

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// nagios.cmd path
    #[clap(short, long)]
    command_file_path: String,

    /// status.dat path
    #[clap(short, long)]
    status_file_path: String,

    #[clap(short, long, default_value_t = 10)]
    max_cache_sec: usize,

    #[clap(short, long)]
    bind_address: Option<String>,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();
    let state: Arc<State> = Arc::new(State::new(
        &args.command_file_path,
        &args.status_file_path,
        args.max_cache_sec,
    ));

    let app = Router::new()
        .route("/", get(handlers::top::handler))
        .route("/hosts/:host_name_regex", get(handlers::hosts::handler))
        .route(
            "/services/:host_name_regex",
            get(handlers::services::handler),
        )
        .route(
            "/cmd/enable_host_notifications/:host_name",
            post(handlers::cmd::enable_host_notifications::handler),
        )
        .route(
            "/cmd/disable_host_notifications/:host_name",
            post(handlers::cmd::disable_host_notifications::handler),
        )
        .route(
            "/cmd/enable_host_check/:host_name",
            post(handlers::cmd::enable_host_check::handler),
        )
        .route(
            "/cmd/disable_host_check/:host_name",
            post(handlers::cmd::disable_host_check::handler),
        )
        .layer(Extension(state));

    axum::Server::bind(
        &args
            .bind_address
            .unwrap_or("0.0.0.0:3000".into())
            .parse()
            .unwrap(),
    )
    .serve(app.into_make_service())
    .await
    .unwrap();
}
