use crate::maze::algorithms::{Algorithm, RecursiveBacktracking};
use crate::maze::OrthogonalMaze;

/// An orthogonal maze builder for constructing a maze step by step
pub struct OrthogonalMazeBuilder {
    width: usize,
    height: usize,
    algorithm: Box<dyn Algorithm>,
}

impl OrthogonalMazeBuilder {
    /// Returns a new instance of a builder with the default width, height and algorithm
    pub fn new() -> Self {
        OrthogonalMazeBuilder {
            width: 10,
            height: 10,
            algorithm: Box::new(RecursiveBacktracking),
        }
    }

    /// Sets a maze width and returns itself
    pub fn width(mut self, width: usize) -> Self {
        self.width = width;
        self
    }

    /// Sets a maze height and returns itself
    pub fn height(mut self, height: usize) -> Self {
        self.height = height;
        self
    }

    /// Sets an algorithm for generating a maze and returns itself
    pub fn algorithm(mut self, algorithm: Box<dyn Algorithm>) -> Self {
        self.algorithm = algorithm;
        self
    }

    /// Builds a maze and returns a resulting object of the generated orthogonal maze
    pub fn build(mut self) -> OrthogonalMaze {
        let mut maze = OrthogonalMaze::new(self.width, self.height);
        self.algorithm.generate(maze.get_grid_mut());
        maze
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn build() {
        let maze = OrthogonalMazeBuilder::new().build();
        assert!(maze.is_valid());
    }
}
