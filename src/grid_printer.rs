use crate::grid::{CellState, Grid};

pub fn print_grid(grid: &Grid) {
    println!("/{}\\", "-".repeat(grid.width()));
    (0..grid.height())
        .map(|y| line_chars(grid, y))
        .for_each(|line| println!("|{}|", line));
    println!("\\{}/", "-".repeat(grid.width()));
}

fn line_chars(grid: &Grid, y: usize) -> String {
    (0..grid.width())
        .map(|x| cell_char(grid, (x, y)))
        .collect::<String>()
}

fn cell_char(grid: &Grid, (x, y): (usize, usize)) -> &str {
    match grid.get_cell((x, y)) {
        CellState::DEAD => " ",
        CellState::ALIVE => "o",
    }
}
