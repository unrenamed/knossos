use crate::maze::algorithms::{Algorithm, RecursiveBacktracking};
use crate::maze::OrthogonalMaze;

pub struct OrthogonalMazeBuilder {
    width: usize,
    height: usize,
    algorithm: Box<dyn Algorithm>,
}

impl OrthogonalMazeBuilder {
    pub fn new() -> Self {
        OrthogonalMazeBuilder {
            width: 10,
            height: 10,
            algorithm: Box::new(RecursiveBacktracking),
        }
    }

    pub fn width(mut self, width: usize) -> Self {
        self.width = width;
        self
    }

    pub fn height(mut self, height: usize) -> Self {
        self.height = height;
        self
    }

    pub fn algorithm(mut self, algorithm: Box<dyn Algorithm>) -> Self {
        self.algorithm = algorithm;
        self
    }

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
