use std::sync::Arc;
use std::sync::Mutex;

use axum::{
    extract::{self, Extension},
    routing::get,
    Router,
};
use nagrs::nagios::Host;

type Nagrs = nagrs::Nagrs<String>;

struct State {
    nagrs: Mutex<Nagrs>,
}

async fn handler(Extension(state): Extension<Arc<State>>) -> String {
    let host: Option<Host>;
    {
        let mut nagrs = state.nagrs.lock().unwrap();
        host = match nagrs.find_host("localhost") {
            Ok(host) => host,
            Err(_) => {
                nagrs.load().unwrap();
                nagrs.find_host("localhost").unwrap()
            }
        };
    }

    std::thread::sleep(std::time::Duration::from_secs(10));
    format!("{:#?}", host)
}

#[tokio::main]
async fn main() {
    let state = Arc::new(State {
        nagrs: Mutex::new(Nagrs::new(
            "./docker/var/rw/nagios.cmd".to_string(),
            "./docker/var/status.dat".to_string(),
        )),
    });

    // build our application with a single route
    let app = Router::new()
        .route("/", get(handler))
        .layer(Extension(state));

    // run it with hyper on localhost:3000
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
