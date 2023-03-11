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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn display_orthogonal_maze() {
        let mut expected = String::new();
        expected.push_str(" _______ \n");
        expected.push_str("| |___  |\n");
        expected.push_str("|_   _| |\n");
        expected.push_str("|  _____|\n");
        expected.push_str("|_______|\n");

        let grid = generate_maze();
        let maze = OrthogonalMaze { grid };
        let actual = maze.to_string();

        assert_eq!(actual, expected);
    }

    #[test]
    fn valid_maze() {
        let grid = generate_maze();
        let maze = OrthogonalMaze { grid };
        assert_eq!(maze.is_valid(), true);
    }

    #[test]
    fn invalid_maze() {
        let mut grid = generate_maze();

        // isolate a top-left cell by adding a South wall
        grid.add_wall((0, 0), Pole::S);

        let maze = OrthogonalMaze { grid };
        assert_eq!(maze.is_valid(), false);
    }

    fn generate_maze() -> Grid {
        let mut grid = Grid::new(4, 4);

        grid.carve_passage((0, 0), Pole::S).unwrap();
        grid.carve_passage((0, 1), Pole::E).unwrap();
        grid.carve_passage((0, 2), Pole::E).unwrap();
        grid.carve_passage((0, 2), Pole::S).unwrap();
        grid.carve_passage((0, 3), Pole::E).unwrap();

        grid.carve_passage((1, 0), Pole::E).unwrap();
        grid.carve_passage((1, 1), Pole::E).unwrap();
        grid.carve_passage((1, 1), Pole::S).unwrap();
        grid.carve_passage((1, 2), Pole::E).unwrap();
        grid.carve_passage((1, 3), Pole::E).unwrap();

        grid.carve_passage((2, 0), Pole::E).unwrap();
        grid.carve_passage((2, 2), Pole::E).unwrap();
        grid.carve_passage((2, 3), Pole::E).unwrap();

        grid.carve_passage((3, 1), Pole::N).unwrap();
        grid.carve_passage((3, 1), Pole::S).unwrap();

        grid
    }
}
