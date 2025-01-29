use super::Algorithm;
use crate::maze::grid::cell::Cell;
use crate::maze::grid::Grid;
use crate::utils::types::Coords;
use clap::ValueEnum;
use rand::prelude::*;

/// An enumeration over supported biases for the "Binary Tree" algorithm
///
/// Each bias represents the two of four sides of the maze that will be spanned
/// by a single corridor.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum Bias {
    /// Produces two long corridors on the Northern and Western sides of the maze
    NorthWest,

    /// Produces two long corridors on the Northern and Eastern sides of the maze
    NorthEast,

    /// Produces two long corridors on the Southern and Western sides of the maze
    SouthWest,

    /// Produces two long corridors on the Southern and Eastern sides of the maze
    SouthEast,
}

/// The "Binary Tree" algorithm for generating mazes
///
/// This is an almost-trivially simple one, but you pay for that simplicity with a few side effects:
/// a notable bias (routes tend to run diagonally) and long corridors spanning two sides. Still,
/// this is quite a performant algorithm since it operates without any state at all looking at the
/// current cell only, without regard for the rest of the cells and rows in the maze.
pub struct BinaryTree {
    bias: Bias,
}

impl BinaryTree {
    /// Create a new instance of the algorithm with a given bias
    ///
    /// # Example
    /// ```
    /// use knossos::maze::{BinaryTree, Bias};
    ///
    /// let algorithm = BinaryTree::new(Bias::NorthWest);
    /// ```
    pub const fn new(bias: Bias) -> BinaryTree {
        BinaryTree { bias }
    }

    fn populate_dirs(&self, coords: Coords, grid: &Grid) -> Vec<Cell> {
        let mut dirs = vec![];
        let (x, y) = coords;

        match self.bias {
            Bias::NorthWest => {
                if y > 0 {
                    dirs.push(Cell::NORTH)
                }
                if x > 0 {
                    dirs.push(Cell::WEST)
                }
            }
            Bias::NorthEast => {
                if y > 0 {
                    dirs.push(Cell::NORTH)
                }
                if x + 1 < grid.width() {
                    dirs.push(Cell::EAST)
                }
            }
            Bias::SouthWest => {
                if y + 1 < grid.height() {
                    dirs.push(Cell::SOUTH)
                }
                if x > 0 {
                    dirs.push(Cell::WEST)
                }
            }
            Bias::SouthEast => {
                if y + 1 < grid.height() {
                    dirs.push(Cell::SOUTH)
                }
                if x + 1 < grid.width() {
                    dirs.push(Cell::EAST)
                }
            }
        }

        dirs
    }
}

/// An implementation of the "Binary Tree" algorithm for generating mazes
///
/// The algorithm is pretty simple: for every cell in the grid, randomly carve a passage either
/// north, or west.
impl Algorithm for BinaryTree {
    fn generate(&mut self, grid: &mut Grid) {
        for y in 0..grid.height() {
            for x in 0..grid.width() {
                let dirs = self.populate_dirs((x, y), grid);
                if let Some(dir) = dirs.choose(&mut rand::rng()) {
                    grid.carve_passage((x, y), *dir).ok();
                }
            }
        }
    }
}
