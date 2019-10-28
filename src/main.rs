use rand::Rng;

use crate::grid::{CellState, Grid};
use crate::grid_printer::print_grid;

mod generation;
mod grid;
mod grid_printer;

fn main() {
    let mut grid = Grid::new((20, 10));
    let mut rng = rand::thread_rng();
    let (width, height) = (grid.width(), grid.height());
    (0..height)
        .flat_map(|y| (0..width).map(move |x| (x, y)))
        .filter(|_| rng.gen_bool(0.3))
        .for_each(|(x, y)| grid.set_cell((x, y), CellState::ALIVE));
    print_grid(&grid);
}
