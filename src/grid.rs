#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum CellState {
    DEAD,
    ALIVE,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Grid {
    width: usize,
    height: usize,
    cells: Vec<CellState>,
}

impl Grid {
    pub fn new((width, height): (usize, usize)) -> Grid {
        assert!(
            width > 2 && height > 2,
            "({}, {}) smaller than (3, 3)",
            width,
            height
        );
        let cells = vec![CellState::DEAD; width * height];
        Grid {
            width,
            height,
            cells,
        }
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn get_cell(&self, (x, y): (usize, usize)) -> CellState {
        self.assert_in_bounds((x, y));
        self.cells[x + y * self.width]
    }

    pub fn get_cell_wrapped(&self, (x, y): (i32, i32)) -> CellState {
        let w = self.width as i32;
        let h = self.height as i32;
        let wrapped_x = (x % w + w) % w;
        let wrapped_y = (y % h + h) % h;
        self.cells[wrapped_x as usize + wrapped_y as usize * self.width]
    }

    pub fn set_cell(&mut self, (x, y): (usize, usize), cell_state: CellState) {
        self.assert_in_bounds((x, y));
        self.cells[x + y * self.width] = cell_state
    }

    fn assert_in_bounds(&self, (x, y): (usize, usize)) {
        assert!(
            x < self.width && y < self.height,
            "({}, {}) out of bounds ({}, {})",
            x,
            y,
            self.width,
            self.height
        );
    }
}

#[cfg(test)]
mod tests {
    use crate::grid::*;

    #[test]
    fn new_grid_size() {
        let grid = Grid::new((31, 42));

        assert_eq!(grid.width, 31);
        assert_eq!(grid.height, 42);
    }

    #[test]
    fn new_grid_min_size() {
        let grid = Grid::new((3, 3));

        assert_eq!(grid.width, 3);
        assert_eq!(grid.height, 3);
    }

    #[test]
    #[should_panic]
    fn new_grid_min_width() {
        Grid::new((2, 3));
    }

    #[test]
    #[should_panic]
    fn new_grid_min_height() {
        Grid::new((3, 2));
    }

    #[test]
    fn get_cell_in_bounds() {
        let grid = Grid::new((4, 3));

        assert_eq!(grid.get_cell((0, 0)), CellState::DEAD);
        assert_eq!(grid.get_cell((1, 0)), CellState::DEAD);
        assert_eq!(grid.get_cell((2, 0)), CellState::DEAD);
        assert_eq!(grid.get_cell((3, 0)), CellState::DEAD);
        assert_eq!(grid.get_cell((0, 1)), CellState::DEAD);
        assert_eq!(grid.get_cell((1, 1)), CellState::DEAD);
        assert_eq!(grid.get_cell((2, 1)), CellState::DEAD);
        assert_eq!(grid.get_cell((3, 1)), CellState::DEAD);
        assert_eq!(grid.get_cell((0, 2)), CellState::DEAD);
        assert_eq!(grid.get_cell((1, 2)), CellState::DEAD);
        assert_eq!(grid.get_cell((2, 2)), CellState::DEAD);
        assert_eq!(grid.get_cell((3, 2)), CellState::DEAD);
    }

    #[test]
    #[should_panic]
    fn get_cell_x_out_of_bounds() {
        let grid = Grid::new((3, 10));

        grid.get_cell((3, 0));
    }

    #[test]
    #[should_panic]
    fn get_cell_y_out_of_bounds() {
        let grid = Grid::new((3, 10));

        grid.get_cell((1, 10));
    }

    #[test]
    fn set_cell_in_bounds() {
        let mut grid = Grid::new((3, 4));

        grid.set_cell((1, 2), CellState::ALIVE);

        assert_eq!(grid.get_cell((1, 2)), CellState::ALIVE);
        assert_eq!(grid.get_cell((2, 1)), CellState::DEAD);
    }

    #[test]
    #[should_panic]
    fn set_cell_x_out_of_bounds() {
        let mut grid = Grid::new((3, 7));

        grid.set_cell((4, 1), CellState::ALIVE);
    }

    #[test]
    #[should_panic]
    fn set_cell_y_out_of_bounds() {
        let mut grid = Grid::new((4, 5));

        grid.set_cell((0, 5), CellState::ALIVE);
    }

    #[test]
    fn get_cell_wrapped_negative_x() {
        let mut grid = Grid::new((3, 3));
        grid.set_cell((2, 0), CellState::ALIVE);

        assert_eq!(grid.get_cell_wrapped((-1, 0)), CellState::ALIVE);
        assert_eq!(grid.get_cell_wrapped((-4, 0)), CellState::ALIVE);
    }

    #[test]
    fn get_cell_wrapped_positive_x() {
        let mut grid = Grid::new((3, 3));
        grid.set_cell((2, 0), CellState::ALIVE);

        assert_eq!(grid.get_cell_wrapped((5, 0)), CellState::ALIVE);
        assert_eq!(grid.get_cell_wrapped((8, 0)), CellState::ALIVE);
    }
}
