use std::sync::Mutex;
use std::{thread, time};

use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use rand::Rng;
use structopt::StructOpt;

use crate::generation::generate_next;
use crate::grid::{CellState, Grid};
use crate::grid_printer::print_grid;

mod generation;
mod grid;
mod grid_printer;
mod json;

#[derive(Debug, StructOpt)]
#[structopt(name = "life", about, author)]
struct Arguments {
    #[structopt(short, long, default_value = "20", name = "grid width")]
    /// Starting grid width, at least 3
    width: usize,
    #[structopt(short, long, default_value = "10", name = "grid height")]
    /// Starting grid height, at least 3
    height: usize,
    #[structopt(short = "a", long = "alive", default_value = "0.3")]
    /// Starting grid ratio of alive cells, between 0 and 1
    alive_ratio: f64,
    #[structopt(short = "f", long = "frequency", default_value = "1.0")]
    /// Cell generation update frequency in Hz
    update_frequency: f64,
    #[structopt(short = "p", long = "port", default_value = "8090")]
    /// Web server port to listen to
    port: u16,
}

struct GridData {
    grid: Grid,
}

fn main() {
    let args: Arguments = Arguments::from_args();
    exit_if(args.width < 3, "width < 3");
    exit_if(args.height < 3, "height < 3");
    exit_if(
        args.alive_ratio < 0.0 || args.alive_ratio > 1.0,
        "alive ratio not between 0 and 1",
    );
    exit_if(
        args.update_frequency <= 0.0 || args.update_frequency > 1000.0,
        "update frequency not included in ]0, 1000]",
    );

    let width = args.width;
    let height = args.height;
    let alive_ratio = args.alive_ratio;
    let sleep_duration = time::Duration::from_secs_f64(1.0 / args.update_frequency);
    let port = args.port;

    let mut grid = Grid::new((width, height));
    let mut rng = rand::thread_rng();
    let (width, height) = (grid.width(), grid.height());
    (0..height)
        .flat_map(|y| (0..width).map(move |x| (x, y)))
        .filter(|_| rng.gen_bool(alive_ratio))
        .for_each(|(x, y)| grid.set_cell((x, y), CellState::ALIVE));

    let data = web::Data::new(Mutex::new(GridData { grid }));
    let logic_data = data.clone();

    let server = HttpServer::new(move || {
        App::new()
            .register_data(data.clone())
            .route("/", web::get().to(index_handler))
            .route("/index.js", web::get().to(js_handler))
            .route("/grid", web::get().to(grid_handler))
    })
    .bind(format!("0.0.0.0:{}", port))
    .expect("Error while binding the socket address");
    thread::spawn(move || loop {
        thread::sleep(sleep_duration);
        {
            let mut guard = logic_data.lock().expect("Error while locking mutex");
            print_grid(&guard.grid);
            guard.grid = generate_next(&guard.grid);
        }
    });
    println!("Server running at http://localhost:{}/", port);
    server.run().expect("Error while running server");
}

fn index_handler() -> impl Responder {
    static INDEX_HTML: &str = include_str!("index.html");
    HttpResponse::Ok().body(INDEX_HTML)
}

fn js_handler() -> impl Responder {
    static INDEX_HTML: &str = include_str!("index.js");
    HttpResponse::Ok().body(INDEX_HTML)
}

fn grid_handler(state: web::Data<Mutex<GridData>>) -> impl Responder {
    let guard = state.lock().expect("Error while locking mutex");
    HttpResponse::Ok()
        .content_type("application/json")
        .body(serde_json::to_string(&guard.grid).expect("Error while serializing grid"))
}

fn exit_if(error_condition: bool, message: &str) {
    if error_condition {
        clap::Error::with_description(message, clap::ErrorKind::InvalidValue).exit()
    }
}
