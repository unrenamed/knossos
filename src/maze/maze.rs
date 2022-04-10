use super::{
    errors::MazeSaveError,
    formatters::{Formatter, Saveable},
    grid::{Grid, pole::Pole},
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
        for row in self.grid.cells() {
            for cell in row {
                let walls = cell.get_walls();
                if !(walls.carved(Pole::N)
                    || walls.carved(Pole::S)
                    || walls.carved(Pole::E)
                    || walls.carved(Pole::W))
                {
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
