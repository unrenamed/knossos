use super::{
    errors::MazeSaveError,
    formatters::{Formatter, Saveable},
    grid::Grid,
};
use std::fmt;

pub struct OrthogonalMaze {
    grid: Grid,
}

impl OrthogonalMaze {
    pub fn new(width: usize, height: usize) -> OrthogonalMaze {
        OrthogonalMaze {
            grid: Grid::new(width, height),
        }
    }

    pub fn get_grid_mut(&mut self) -> &mut Grid {
        &mut self.grid
    }

    pub fn is_valid(&self) -> bool {
        for y in 0..self.grid.height() {
            for x in 0..self.grid.width() {
                if !self.grid.is_cell_visited((x, y)) {
                    return false;
                }
            }
        }

        true
    }

    pub fn save<F, T>(&self, path: &str, formatter: F) -> Result<String, MazeSaveError>
    where
        F: Formatter<T>,
        T: Saveable,
    {
        let data = formatter.format(&self.grid);
        Saveable::save(&data, path)
    }
}

impl fmt::Display for OrthogonalMaze {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.grid)?;
        Ok(())
    }
}
