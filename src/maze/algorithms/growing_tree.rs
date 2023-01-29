use super::Algorithm;
use crate::maze::grid::{
    pole::Pole::{E, N, S, W},
    Grid,
};
use crate::utils::types::Coords;
use clap::ValueEnum;
use rand::prelude::*;

/// An enumeration over supported cell selection methods for the "Growing Tree" algorithm
///
/// Each method represents the way a new cell is selected causing the "Growing Tree" algorithm
/// to imitate other algorithms or theirs combinations
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum Method {
    /// Selects the most recently added cell, thus imitating the recurive backtracker
    Newest,

    /// Selects the oldest added cell, thus generating an unchallenging maze with lots of long
    /// corridors
    Oldest,

    /// Selects cells at random, thus getting Prim's algorithm behaviour
    Random,

    /// Selects a middle cell from the list of already added, but produces mazes similar to the
    /// ones created by the [Oldest](Method::Oldest) method
    Middle,

    /// A combination of the [Newest](Method::Newest) and [Random](Method::Random) methods with
    /// 50/50 split
    Newest50Random50,

    /// A combination of the [Newest](Method::Newest) and [Random](Method::Random) methods with
    /// 75/25 split
    Newest75Random25,

    /// A combination of the [Newest](Method::Newest) and [Random](Method::Random) methods with
    /// 25/75 split
    Newest25Random75,
}

/// The "Growing Tree" algorithm for generating mazes
///
/// The algorithm appears to be the most flexible and customizable. Configured one way,
/// it mimics the behavior of the Recursive Backtracking algorithm. Configured another,
/// it works almost exactly like Prim's algorithm. Another trivial change and you can
/// combine two or more methods with some probability and generate mazes with mixed attributes.
///
/// You can find plenty of supported methods in the [Method] enum.
pub struct GrowingTree {
    method: Method,
}

impl GrowingTree {
    /// Create a new instance of the algorithm with a given method
    ///
    /// # Example
    /// ```
    /// use knossos::maze::{GrowingTree, Method};
    ///
    /// let algorithm = GrowingTree::new(Method::Newest);
    /// ```
    pub fn new(method: Method) -> GrowingTree {
        GrowingTree { method }
    }

    fn choose_index(&self, ceil: usize) -> usize {
        let mut rng = rand::thread_rng();

        match self.method {
            Method::Oldest => 0,
            Method::Newest => ceil - 1,
            Method::Middle => ceil / 2,
            Method::Random => rng.gen_range(0..ceil),
            Method::Newest50Random50 => {
                let is_newest: bool = rng.gen();
                if is_newest {
                    ceil - 1
                } else {
                    rng.gen_range(0..ceil)
                }
            }
            Method::Newest75Random25 => {
                let prob: f32 = rng.gen();
                if prob < 0.75 {
                    ceil - 1
                } else {
                    rng.gen_range(0..ceil)
                }
            }
            Method::Newest25Random75 => {
                let prob: f32 = rng.gen();
                if prob < 0.25 {
                    ceil - 1
                } else {
                    rng.gen_range(0..ceil)
                }
            }
        }
    }
}

/// An implementation of the "Growing Tree" algorithm for generating mazes
///
/// Despite the method you selected, the algorithm steps remain the same and pretty slick. Here is
/// how it works:
///
/// 1. Initializes an empty list of cells (hereinafter the C)
///
/// 2. Adds one cell to the C, at random
///
/// 3. Chooses a cell from the C and carves a passage to any unvisited neighbor of that cell
/// adding that neighbor to the C as well. If there are no unvisited neighbors, removes the cell
/// from the C
///
/// 4. Repeats #3 until the C is empty
impl Algorithm for GrowingTree {
    fn generate(&mut self, grid: &mut Grid) {
        let mut directions = [E, N, S, W];
        let mut cells = vec![];
        cells.push(get_rand_coords(grid));

        while cells.len() > 0 {
            let mut index = Some(self.choose_index(cells.len()));
            let coords = cells[index.unwrap_or(0)];

            directions.shuffle(&mut rand::thread_rng());
            for dir in directions {
                let next = match grid.get_next_cell_coords(coords, dir) {
                    Ok(next) => next,
                    Err(_) => continue,
                };

                if grid.is_cell_visited(next) {
                    continue;
                }

                if let Ok(next) = grid.carve_passage(coords, dir) {
                    cells.push(next);
                    index = None;
                    break;
                }
            }

            if let Some(index) = index {
                cells.remove(index);
            }
        }
    }
}

fn get_rand_coords(grid: &Grid) -> Coords {
    let x = rand::thread_rng().gen_range(0..grid.width());
    let y = rand::thread_rng().gen_range(0..grid.height());
    (x, y)
}
