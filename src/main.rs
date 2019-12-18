use std::sync::{Arc, Mutex};
use std::{thread, time};

use rand::Rng;
use structopt::StructOpt;

use crate::generation::generate_next;
use crate::grid::{CellState, Grid};
use crate::grid_printer::print_grid;
use crate::server::run_server;
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::Duration;

mod generation;
mod grid;
mod grid_printer;
mod json;
mod server;

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

#[derive(Debug)]
pub struct State {
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

    let grid = build_random_grid(width, height, alive_ratio);

    let logic_state = Arc::new(Mutex::new(State { grid }));
    let server_state = logic_state.clone();
    let thread_running = Arc::new(AtomicBool::new(true));
    let thread_running_ref = thread_running.clone();

    let logic_handle =
        thread::spawn(move || run_logic(sleep_duration, logic_state, thread_running));
    let server_handle = thread::spawn(move || run_server(port, server_state));

    server_handle
        .join()
        .expect("Error while waiting for server thread");
    thread_running_ref.store(false, Ordering::Relaxed);
    logic_handle
        .join()
        .expect("Error while waiting for logic thread");
}

fn build_random_grid(width: usize, height: usize, alive_ratio: f64) -> Grid {
    let mut grid = Grid::new((width, height));
    let mut rng = rand::thread_rng();
    let (width, height) = (grid.width(), grid.height());
    (0..height)
        .flat_map(|y| (0..width).map(move |x| (x, y)))
        .filter(|_| rng.gen_bool(alive_ratio))
        .for_each(|(x, y)| grid.set_cell((x, y), CellState::ALIVE));
    grid
}

fn run_logic(
    sleep_duration: Duration,
    logic_state: Arc<Mutex<State>>,
    thread_running: Arc<AtomicBool>,
) {
    while thread_running.load(Ordering::Relaxed) {
        thread::sleep(sleep_duration);
        {
            let mut guard = logic_state.lock().expect("Error while locking mutex");
            print_grid(&guard.grid);
            guard.grid = generate_next(&guard.grid);
        }
    }
}

fn exit_if(error_condition: bool, message: &str) {
    if error_condition {
        clap::Error::with_description(message, clap::ErrorKind::InvalidValue).exit()
    }
}
