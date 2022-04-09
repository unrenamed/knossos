use super::Algorithm;
use crate::maze::grid::{pole::Pole, Grid};
use rand::prelude::*;

/// The "Sidewinder" algorithm for generating mazes
///
/// It’s closely related to the "Binary Tree" algorithm, but manages to get away with
/// only one side being spanned by a passage, instead of two. Space- and performance-wise,
/// the algorithm is quite efficient since it looks at one row at a time.
pub struct Sidewinder;

/// An implementation of the "Sidewinder" algorithm for generating mazes
///
/// The steps are as follows:
/// 1. Works through the grid row-wise, starting with the cell at 0,0. Initializes the “run” set to be empty
/// 2. Adds the current cell to the “run” set
/// 3. For the current cell, randomly decides whether to carve east or not
/// 4. If a passage was carved, makes the new cell the current cell and repeats steps 2-4
/// 5. If a passage was not carved, chooses any one of the cells in the run set and carves a passage north.
/// Then empties the run set, sets the next cell in the row to be the current cell, and repeats steps 2-5
/// 6. Continues until all rows have been processed
impl Algorithm for Sidewinder {
    fn generate(&mut self, grid: &mut Grid) {
        let mut rng = rand::thread_rng();

        for y in 0..grid.height() {
            let mut run_start = 0;

            for x in 0..grid.width() {
                let carve_east: bool = rng.gen();

                if y == 0 || (carve_east && x + 1 < grid.width()) {
                    grid.carve_passage((x, y), Pole::E).ok();
                } else {
                    let rand_x = rng.gen_range(run_start..=x);
                    grid.carve_passage((rand_x, y), Pole::N).ok();
                    run_start = x + 1;
                }
            }
        }
    }
}
