use crate::maze::algorithms::{Algorithm, RecursiveBacktracking};
use crate::maze::OrthogonalMaze;
use crate::utils::types::Coords;

use super::errors::BuildError;

/// An orthogonal maze builder for constructing a maze step by step
pub struct OrthogonalMazeBuilder {
    width: usize,
    height: usize,
    algorithm: Box<dyn Algorithm>,
    start_coords: Option<Coords>,
}

impl OrthogonalMazeBuilder {
    /// Returns a new instance of a builder with the default width, height and algorithm
    pub fn new() -> Self {
        OrthogonalMazeBuilder {
            width: 10,
            height: 10,
            algorithm: Box::new(RecursiveBacktracking),
            start_coords: None,
        }
    }

    /// Sets a maze width and returns itself
    pub const fn width(mut self, width: usize) -> Self {
        self.width = width;
        self
    }

    /// Sets a maze height and returns itself
    pub const fn height(mut self, height: usize) -> Self {
        self.height = height;
        self
    }

    /// Sets an algorithm for generating a maze and returns itself
    pub fn algorithm(mut self, algorithm: Box<dyn Algorithm>) -> Self {
        self.algorithm = algorithm;
        self
    }

    /// Sets start coords for arguments that allow start coords
    pub fn start_coords(mut self, coord: impl Into<Coords>) -> Self {
        self.start_coords = Some(coord.into());
        self
    }

    /// Builds a maze and returns a resulting object of the generated orthogonal maze
    pub fn build(mut self) -> Result<OrthogonalMaze, BuildError> {
        let mut maze = OrthogonalMaze::new(self.width, self.height);
        if self.start_coords.is_some() && !self.algorithm.has_start_coords() {
            Err(BuildError::reason(self.algorithm.name()))
        } else {
            self.algorithm
                .generate(maze.get_grid_mut(), self.start_coords);
            Ok(maze)
        }
    }
}

impl Default for OrthogonalMazeBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use crate::maze::RecursiveDivision;

    use super::*;

    #[test]
    fn build() {
        let maze = OrthogonalMazeBuilder::default().build().unwrap();
        assert!(maze.is_valid());
    }

    #[test]
    fn no_start_coord_support() {
        let maze_err = OrthogonalMazeBuilder::default()
            .start_coords((3, 3))
            .algorithm(Box::new(RecursiveDivision {}))
            .build()
            .unwrap_err();
        assert_eq!(maze_err.to_string(), "Cannot build maze. Reason: Algorithm `RecursiveDivision` doesn't support `start_coords`");
    }
}
