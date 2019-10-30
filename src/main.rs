use std::{thread, time};

use rand::Rng;
use structopt::StructOpt;

use crate::generation::generate_next;
use crate::grid::{CellState, Grid};
use crate::grid_printer::print_grid;

mod generation;
mod grid;
mod grid_printer;

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
}

fn main() {
    let args = Arguments::from_args();
    exit_if(args.width < 3, "width < 3");
    exit_if(args.height < 3, "height < 3");
    exit_if(
        args.alive_ratio < 0.0 || args.alive_ratio > 1.0,
        "alive ratio not between 0 and 1",
    );

    let width = args.width;
    let height = args.height;
    let alive_ratio = args.alive_ratio;

    let mut grid = Grid::new((width, height));
    let mut rng = rand::thread_rng();
    let (width, height) = (grid.width(), grid.height());
    (0..height)
        .flat_map(|y| (0..width).map(move |x| (x, y)))
        .filter(|_| rng.gen_bool(alive_ratio))
        .for_each(|(x, y)| grid.set_cell((x, y), CellState::ALIVE));
    loop {
        print_grid(&grid);
        thread::sleep(time::Duration::from_millis(250));
        grid = generate_next(&grid);
    }
}

fn exit_if(error_condition: bool, message: &str) {
    if error_condition {
        clap::Error::with_description(message, clap::ErrorKind::InvalidValue).exit()
    }
}
