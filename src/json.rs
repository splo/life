use core::fmt;

use serde::de::{Deserialize, Deserializer};
use serde::ser::{Serialize, SerializeMap, Serializer};

use crate::grid::*;

impl Serialize for Grid {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut map = serializer.serialize_map(Some(3))?;
        map.serialize_entry("width", &self.width())?;
        map.serialize_entry("height", &self.height())?;
        let mut cells: Vec<Vec<bool>> = Vec::new();
        (0..self.height()).for_each(|y| {
            let mut line: Vec<bool> = Vec::new();
            (0..self.width()).for_each(|x| {
                line.push(self.get_cell((x, y)) == CellState::ALIVE);
            });
            cells.push(line);
        });
        map.serialize_entry("cells", &cells)?;
        map.end()
    }
}

impl<'de> Deserialize<'de> for Grid {
    fn deserialize<D>(deserializer: D) -> Result<Grid, D::Error>
    where
        D: Deserializer<'de>,
    {
        use serde::de::{MapAccess, Visitor};

        struct GridVisitor;
        impl<'de> Visitor<'de> for GridVisitor {
            type Value = Grid;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a map")
            }

            #[inline]
            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'de>,
            {
                let mut width: usize = 3;
                let mut height: usize = 3;
                let mut cells: Vec<Vec<bool>> = Vec::new();
                while let Some(key) = map.next_key::<String>()? {
                    match key.as_str() {
                        "width" => width = map.next_value()?,
                        "height" => height = map.next_value()?,
                        "cells" => cells = map.next_value()?,
                        _ => (),
                    }
                }
                let mut grid = Grid::new((width, height));
                (0..height).for_each(|y| {
                    (0..width).for_each(|x| {
                        grid.set_cell(
                            (x, y),
                            if *cells.get(y).unwrap().get(x).unwrap() {
                                CellState::ALIVE
                            } else {
                                CellState::DEAD
                            },
                        );
                    });
                });
                Ok(grid)
            }
        }

        deserializer.deserialize_map(GridVisitor)
    }
}

#[cfg(test)]
mod tests {
    use crate::grid::*;

    #[test]
    fn serialization() {
        let mut grid = Grid::new((3, 4));
        grid.set_cell((2, 0), CellState::ALIVE);
        grid.set_cell((0, 3), CellState::ALIVE);
        grid.set_cell((1, 3), CellState::ALIVE);
        grid.set_cell((2, 3), CellState::ALIVE);
        grid.set_cell((2, 3), CellState::ALIVE);

        let grid_str = serde_json::to_string(&grid).unwrap();
        let actual_value: serde_json::Value = serde_json::from_str(&grid_str).unwrap();
        let expected_value: serde_json::Value = serde_json::json!({
        "width": 3,
        "height": 4,
        "cells": [
        [false, false, true],
        [false, false, false],
        [false, false, false],
        [true, true, true]
        ]
        });
        assert_eq!(&actual_value, &expected_value);
    }

    #[test]
    fn deserialization() {
        let value: serde_json::Value = serde_json::json!({
        "width": 3,
        "height": 4,
        "cells": [
        [false, false, true],
        [true, true, true],
        [false, false, false],
        [true, false, false]
        ]
        });

        let actual_grid: Grid = serde_json::from_value(value).unwrap();

        let mut expected_grid = Grid::new((3, 4));
        expected_grid.set_cell((2, 0), CellState::ALIVE);
        expected_grid.set_cell((0, 1), CellState::ALIVE);
        expected_grid.set_cell((1, 1), CellState::ALIVE);
        expected_grid.set_cell((2, 1), CellState::ALIVE);
        expected_grid.set_cell((2, 1), CellState::ALIVE);
        expected_grid.set_cell((0, 3), CellState::ALIVE);
        assert_eq!(&actual_grid, &expected_grid);
    }
}
