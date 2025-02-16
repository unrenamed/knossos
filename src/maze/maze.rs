use super::{
    errors::MazeSaveError,
    formatters::{Formatter, Saveable},
    grid::Grid,
    validate::validate,
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
    pub(crate) fn get_grid_mut(&mut self) -> &mut Grid {
        &mut self.grid
    }

    /// Returns `true` if a maze is valid. Otherwise, returns `false`
    pub fn is_valid(&self) -> bool {
        validate(&self.grid)
    }

    /// Saves the maze data to a file at the specified path using the provided formatter.
    ///
    /// This method converts the internal grid representation of the maze into a format
    /// defined by the `Formatter` and `Saveable` trait implementations, and then saves it
    /// to the specified file path.
    ///
    /// # Example
    /// ```rust
    /// use knossos::maze::*;
    ///
    /// let maze = OrthogonalMaze::new(5, 5);
    /// maze.save("output/maze.txt", AsciiNarrow);
    /// ```
    ///
    /// # Errors
    /// This function can return a `MazeSaveError` if the file could not be written.
    pub fn save<F, T>(&self, path: &str, formatter: F) -> Result<String, MazeSaveError>
    where
        F: Formatter<T>,
        T: Saveable,
    {
        let data = formatter.format(&self.grid);
        Saveable::save(&data, path)
    }

    /// Returns a formatted maze using the provided formatter.
    ///
    /// This method generates a formatted representation of the maze grid by applying
    /// the specified formatter, which converts the internal maze structure into the
    /// desired format `T`. This formatted result can then be further processed or saved
    /// elsewhere.
    ///
    /// # Example
    /// ```rust
    /// use knossos::maze::*;
    ///
    /// let maze = OrthogonalMaze::new(5, 5);
    /// let formatted_maze = maze.format(GameMap::new());
    /// ```
    ///
    /// # Note
    /// This function can be useful when the formatted maze needs to be used for further processing,
    /// logging, or display.
    pub fn format<F, T>(&self, formatter: F) -> T
    where
        F: Formatter<T>,
        T: Saveable,
    {
        formatter.format(&self.grid)
    }
}

impl fmt::Display for OrthogonalMaze {
    /// Writes a formatted maze into a buffer
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.grid)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::maze::grid::cell::Cell;

    use super::*;

    #[test]
    fn display_orthogonal_maze() {
        let mut expected = String::new();
        expected.push_str(" _______ \n");
        expected.push_str("| |___  |\n");
        expected.push_str("|_   _| |\n");
        expected.push_str("|  _____|\n");
        expected.push_str("|_______|\n");

        let grid = generate_valid_maze();
        let maze = OrthogonalMaze { grid };
        let actual = maze.to_string();

        assert_eq!(actual, expected);
    }

    #[test]
    fn valid_maze() {
        let grid = generate_valid_maze();
        let maze = OrthogonalMaze { grid };
        assert!(maze.is_valid());
    }

    #[test]
    fn invalid_maze() {
        let grid = generate_invalid_maze();
        let maze = OrthogonalMaze { grid };
        assert!(!maze.is_valid());
    }

    fn generate_valid_maze() -> Grid {
        let mut grid = Grid::new(4, 4);

        grid.carve_passage((0, 0), Cell::SOUTH).unwrap();
        grid.carve_passage((0, 1), Cell::EAST).unwrap();
        grid.carve_passage((0, 2), Cell::EAST).unwrap();
        grid.carve_passage((0, 2), Cell::SOUTH).unwrap();
        grid.carve_passage((0, 3), Cell::EAST).unwrap();

        grid.carve_passage((1, 0), Cell::EAST).unwrap();
        grid.carve_passage((1, 1), Cell::EAST).unwrap();
        grid.carve_passage((1, 1), Cell::SOUTH).unwrap();
        grid.carve_passage((1, 2), Cell::EAST).unwrap();
        grid.carve_passage((1, 3), Cell::EAST).unwrap();

        grid.carve_passage((2, 0), Cell::EAST).unwrap();
        grid.carve_passage((2, 2), Cell::EAST).unwrap();
        grid.carve_passage((2, 3), Cell::EAST).unwrap();

        grid.carve_passage((3, 1), Cell::NORTH).unwrap();
        grid.carve_passage((3, 1), Cell::SOUTH).unwrap();

        grid
    }

    fn generate_invalid_maze() -> Grid {
        let mut grid = Grid::new(4, 4);

        grid.carve_passage((0, 0), Cell::SOUTH).unwrap();
        grid.carve_passage((0, 1), Cell::EAST).unwrap();
        grid.carve_passage((0, 2), Cell::EAST).unwrap();
        grid.carve_passage((0, 2), Cell::SOUTH).unwrap();
        grid.carve_passage((0, 3), Cell::EAST).unwrap();

        grid.carve_passage((1, 1), Cell::EAST).unwrap();
        grid.carve_passage((1, 1), Cell::NORTH).unwrap();
        grid.carve_passage((1, 2), Cell::EAST).unwrap();
        grid.carve_passage((1, 3), Cell::EAST).unwrap();

        grid.carve_passage((2, 0), Cell::EAST).unwrap();
        grid.carve_passage((2, 2), Cell::EAST).unwrap();
        grid.carve_passage((2, 3), Cell::EAST).unwrap();

        grid.carve_passage((3, 1), Cell::NORTH).unwrap();
        grid.carve_passage((3, 1), Cell::SOUTH).unwrap();

        grid
    }
}
