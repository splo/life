use crate::grid::{CellState, Grid};

pub fn generate_next(grid: &Grid) -> Grid {
    let mut new_grid = Grid::new((grid.width(), grid.height()));
    (0..grid.height())
        .flat_map(|y| (0..grid.width()).map(move |x| (x, y)))
        .for_each(|(x, y)| {
            let neighbors = count_neighbors(grid, (x, y));
            let new_state = match grid.get_cell((x, y)) {
                // Born if 2 alive cells around.
                CellState::Dead if (neighbors == 3) => CellState::Alive,
                // Survives if 2 or 3 alive cells around.
                CellState::Alive if (neighbors == 2 || neighbors == 3) => CellState::Alive,
                // Else dies or stays dead.
                _ => CellState::Dead,
            };
            new_grid.set_cell((x, y), new_state)
        });
    new_grid
}

fn count_neighbors(grid: &Grid, (x, y): (usize, usize)) -> usize {
    let x = x as i32;
    let y = y as i32;
    (y - 1..y + 2)
        .flat_map(|j| (x - 1..x + 2).map(move |i| (i, j)))
        .filter(|&(i, j)| i != x || j != y)
        .filter(|&(i, j)| grid.get_cell_wrapped((i, j)) == CellState::Alive)
        .count()
}

#[cfg(test)]
mod tests {
    use crate::generation::*;
    use crate::grid::*;

    #[test]
    fn count_neighbors_with_no_alive_cell() {
        let grid = Grid::new((4, 4));

        assert_eq!(count_neighbors(&grid, (0, 0)), 0);
    }

    #[test]
    fn count_neighbors_with_3_alive_cells() {
        let mut grid = Grid::new((4, 4));
        grid.set_cell((0, 0), CellState::Alive);
        grid.set_cell((0, 1), CellState::Alive);
        grid.set_cell((1, 0), CellState::Alive);
        /*
        /----\
        |oo  |
        |o   |
        |    |
        |    |
        \----/
        */
        assert_eq!(count_neighbors(&grid, (0, 0)), 2);
        assert_eq!(count_neighbors(&grid, (1, 0)), 2);
        assert_eq!(count_neighbors(&grid, (2, 0)), 1);
        assert_eq!(count_neighbors(&grid, (3, 0)), 2);
        assert_eq!(count_neighbors(&grid, (2, 2)), 0);
    }

    #[test]
    fn generate_next_given_blinker() {
        let mut grid = Grid::new((5, 5));
        grid.set_cell((1, 2), CellState::Alive);
        grid.set_cell((2, 2), CellState::Alive);
        grid.set_cell((3, 2), CellState::Alive);

        let new_grid = generate_next(&grid);

        assert_eq!(new_grid.get_cell((0, 0)), CellState::Dead);
        assert_eq!(new_grid.get_cell((1, 0)), CellState::Dead);
        assert_eq!(new_grid.get_cell((2, 0)), CellState::Dead);
        assert_eq!(new_grid.get_cell((3, 0)), CellState::Dead);
        assert_eq!(new_grid.get_cell((4, 0)), CellState::Dead);
        assert_eq!(new_grid.get_cell((0, 1)), CellState::Dead);
        assert_eq!(new_grid.get_cell((1, 1)), CellState::Dead);
        assert_eq!(new_grid.get_cell((2, 1)), CellState::Alive);
        assert_eq!(new_grid.get_cell((3, 1)), CellState::Dead);
        assert_eq!(new_grid.get_cell((4, 1)), CellState::Dead);
        assert_eq!(new_grid.get_cell((0, 2)), CellState::Dead);
        assert_eq!(new_grid.get_cell((1, 2)), CellState::Dead);
        assert_eq!(new_grid.get_cell((2, 2)), CellState::Alive);
        assert_eq!(new_grid.get_cell((3, 2)), CellState::Dead);
        assert_eq!(new_grid.get_cell((4, 2)), CellState::Dead);
        assert_eq!(new_grid.get_cell((0, 3)), CellState::Dead);
        assert_eq!(new_grid.get_cell((1, 3)), CellState::Dead);
        assert_eq!(new_grid.get_cell((2, 3)), CellState::Alive);
        assert_eq!(new_grid.get_cell((3, 3)), CellState::Dead);
        assert_eq!(new_grid.get_cell((4, 3)), CellState::Dead);
        assert_eq!(new_grid.get_cell((0, 4)), CellState::Dead);
        assert_eq!(new_grid.get_cell((1, 4)), CellState::Dead);
        assert_eq!(new_grid.get_cell((2, 4)), CellState::Dead);
        assert_eq!(new_grid.get_cell((3, 4)), CellState::Dead);
        assert_eq!(new_grid.get_cell((4, 4)), CellState::Dead);
    }
}
