use axum::{
    http::{self, Method},
    routing::post,
};
use std::sync::Arc;

use axum::{extract::Extension, routing::get, Router};
use clap::Parser;
use nagotiator::{handlers, state::State};
use tower_http::cors::{Any, CorsLayer};

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
    load_interval_sec: u64,

    #[clap(short, long)]
    bind_address: Option<String>,

    /// comma separated
    #[clap(short, long)]
    allow_origins: Option<String>,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();

    let raw_state = State::new(
        &args.command_file_path,
        &args.status_file_path,
        args.load_interval_sec,
    );

    let state: Arc<State> = Arc::new(raw_state);

    // load status.dat every load_interval_sec
    let load_state = state.clone();
    let load_loop = tokio::task::spawn(async move {
        let mut interval =
            tokio::time::interval(std::time::Duration::from_secs(load_state.load_interval_sec));
        loop {
            interval.tick().await;
            let result = load_state.load();
            match result {
                Ok(_) => println!("load success!"),
                Err(error) => println!("load faield: {}", error),
            }
        }
    });

    let cors = CorsLayer::new()
        .allow_headers(vec![http::header::CONTENT_TYPE])
        .allow_methods([Method::GET, Method::POST])
        .allow_origin(Any); // TODO configurable

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
        .route(
            "/cmd/enable_svc_check/:host_name/:service_description",
            post(handlers::cmd::enable_svc_check::handler),
        )
        .route(
            "/cmd/disable_svc_check/:host_name/:service_description",
            post(handlers::cmd::disable_svc_check::handler),
        )
        .route(
            "/cmd/enable_svc_notifications/:host_name/:service_description",
            post(handlers::cmd::enable_svc_notifications::handler),
        )
        .route(
            "/cmd/disable_svc_notifications/:host_name/:service_description",
            post(handlers::cmd::disable_svc_notifications::handler),
        )
        .route(
            "/cmd/enable_host_svc_checks/:host_name",
            post(handlers::cmd::enable_host_svc_checks::handler),
        )
        .route(
            "/cmd/disable_host_svc_checks/:host_name",
            post(handlers::cmd::disable_host_svc_checks::handler),
        )
        .route(
            "/cmd/enable_host_svc_notifications/:host_name",
            post(handlers::cmd::enable_host_svc_notifications::handler),
        )
        .route(
            "/cmd/disable_host_svc_notifications/:host_name",
            post(handlers::cmd::disable_host_svc_notifications::handler),
        )
        .layer(Extension(state))
        .layer(cors);

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

    load_loop.await.unwrap();
}
