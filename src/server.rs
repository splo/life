use std::sync::{Arc, Mutex};

use actix_web::{web, App, HttpResponse, HttpServer, Responder};

use crate::State;

fn index_handler() -> impl Responder {
    static INDEX_HTML: &str = include_str!("index.html");
    HttpResponse::Ok().body(INDEX_HTML)
}

fn js_handler() -> impl Responder {
    static INDEX_HTML: &str = include_str!("index.js");
    HttpResponse::Ok().body(INDEX_HTML)
}

fn grid_handler(state: web::Data<Arc<Mutex<State>>>) -> impl Responder {
    let guard = state.lock().expect("Error while locking mutex");
    HttpResponse::Ok()
        .content_type("application/json")
        .body(serde_json::to_string(&guard.grid).expect("Error while serializing grid"))
}

pub fn run_server(port: u16, data: Arc<Mutex<State>>) {
    let server = HttpServer::new(move || {
        App::new()
            .data(data.clone())
            .wrap(actix_web::middleware::Logger::default())
            .route("/", web::get().to(index_handler))
            .route("/index.js", web::get().to(js_handler))
            .route("/grid", web::get().to(grid_handler))
    })
    .bind(format!("0.0.0.0:{}", port))
    .expect("Error while binding the socket address");
    println!("Server running at http://localhost:{}/", port);
    server.run().expect("Error while running server");
}
