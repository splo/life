use std::{
    net::SocketAddr,
    str::FromStr,
    sync::{Arc, Mutex},
};

use axum::{
    response::{Html, IntoResponse},
    routing::get,
    Extension, Json, Router,
};
use tokio::net::TcpListener;

use crate::State;

async fn index_handler() -> impl IntoResponse {
    static INDEX_HTML: &str = include_str!("index.html");
    Html(INDEX_HTML)
}

async fn js_handler() -> impl IntoResponse {
    static INDEX_JS: &str = include_str!("index.js");
    Html(INDEX_JS)
}

async fn grid_handler(Extension(state): Extension<Arc<Mutex<State>>>) -> impl IntoResponse {
    let guard = state.lock().expect("Error while locking mutex");
    Json(guard.grid.clone())
}

pub fn run_server(port: u16, data: Arc<Mutex<State>>) {
    let app = Router::new()
        .route("/", get(index_handler))
        .route("/index.js", get(js_handler))
        .route("/grid", get(grid_handler).layer(Extension(data)));
    let address = &&SocketAddr::from_str((format!("0.0.0.0:{}", port)).as_str())
        .expect("Cannot parse address");
    println!("Server running at http://localhost:{}/", port);
    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .expect("Error while starting Tokio runtime")
        .block_on(async {
            let listener = TcpListener::bind(address)
                .await
                .expect("Error while binding TCP socket");
            axum::serve(listener, app.into_make_service())
                .await
                .expect("Error while running server");
        });
}
