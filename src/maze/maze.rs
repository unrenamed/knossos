use super::{
    errors::MazeSaveError,
    formatters::{Formatter, Saveable},
    grid::Grid,
};
use std::fmt;

/// An orthogonal maze
///
/// Represents a standard orthogonal maze where each cell is a square containing zero or maximum
/// three walls
pub struct OrthogonalMaze {
    grid: Grid,
}

impl OrthogonalMaze {
    /// Returns a new instance of an orthogonal maze with a given width and height
    pub fn new(width: usize, height: usize) -> OrthogonalMaze {
        OrthogonalMaze {
            grid: Grid::new(width, height),
        }
    }

    /// Returns a mutable ref to a grid
    pub fn get_grid_mut(&mut self) -> &mut Grid {
        &mut self.grid
    }

    /// Returns `true` if a maze is valid. Otherwise, returns `false`
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

    // Saves a maze into a file to a given path using a given formatter
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
    /// Writes a formatted maze into a buffer
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.grid)?;
        Ok(())
    }
}
