use super::Algorithm;
use crate::maze::grid::{cell::Cell, Grid};
use rand::prelude::*;

enum Orientation {
    Horizontal,
    Vertical,
}

/// The "Recursive Division" algorithm for generating mazes
///
/// According to [Wikipedia](https://en.wikipedia.org/wiki/Maze_generation_algorithm#Recursive_division_method)
/// and [James Buck's maze generation blogs](https://weblog.jamisbuck.org/2011/1/12/maze-generation-recursive-division-algorithm)
/// this algorithm should be implemented as a wall adder, as opposed to the rest of the algorithms
/// in this library which are designated as "passage carvers". In a nutshell, the idea is to
/// recursively divide the original grid with no walls — call this a "chamber" — into smaller
/// subchambers with randomly positioned walls and single passages within them. This fractal nature
/// makes this algorithm novel. You could theoretically continue the process indefinitely at
/// progressively finer and finer levels of detail on demand. This may be useful when developing a
/// game where a player wanders from one section of the maze to another without a need to store the
/// entire maze in the memory.
///
/// My implementation of this algorithm is a bit different from the above. The design of this
/// library supports only the passage-carving technique (mixing both techniques would lead to an
/// uglier API and less efficient generation mechanism, which I tested using benchmarks). Given that
/// I've slightly modified the algorithm to be rather a "passage carver". The method and its fractal
/// nature remain though. Of course, the generation process would be less fancy this way, but this
/// library has nothing to do with visualizing the generation progress step-by-step. If you're
/// interested in this side of the maze generation world, feel free to check my other program for
/// the terminal called [Daedalus](https://github.com/unrenamed/daedalus).
///
/// It's also worth mentioning that the algorithm's fractal nature leads to some visual artifacts
/// and bottlenecks like a single passage between two sections that effectively divide the entire
/// maze into two distinct regions, thus making it easy to spot the passage and work backward to a
/// solution.
pub struct RecursiveDivision;

impl RecursiveDivision {
    fn divide(grid: &mut Grid, x: usize, y: usize, ax: usize, ay: usize) {
        // Calculate subfield width
        let w = ax - x + 1;
        // Calculate subfield height
        let h = ay - y + 1;

        if w < 2 || h < 2 {
            if w > 1 {
                // Carve passages till the horizontal end of the subfield
                for cx in x..ax {
                    grid.carve_passage((cx, y), Cell::EAST).unwrap();
                }
            } else if h > 1 {
                // Carve passages till the vertical end of the subfield
                for cy in y..ay {
                    grid.carve_passage((x, cy), Cell::SOUTH).unwrap();
                }
            }
            return;
        }

        let mut rng = rand::thread_rng();

        // Which way a subfield with the given dimensions ought to be bisected
        let orientation = choose_orientation(w, h);

        // Get X and Y coordinates of a cell where a passage will be carved
        let px = rng.gen_range(x..ax);
        let py = rng.gen_range(y..ay);

        // Define what direction is corresponding to the wall orientation
        let dir = match orientation {
            Orientation::Horizontal => Cell::SOUTH,
            Orientation::Vertical => Cell::EAST,
        };

        // Carve passage
        let (nx, ny) = grid.carve_passage((px, py), dir).unwrap();

        // Determine the bounds of the subfields and get them split
        match orientation {
            Orientation::Horizontal => {
                // Top subfield
                RecursiveDivision::divide(grid, x, y, ax, py);
                // Bottom subfield
                RecursiveDivision::divide(grid, x, ny, ax, ay);
            }
            Orientation::Vertical => {
                // Left subfield
                RecursiveDivision::divide(grid, x, y, px, ay);
                // Right subfield
                RecursiveDivision::divide(grid, nx, y, ax, ay);
            }
        }
    }
}

/// An implementation of the "Recursive Division" algorithm for generating mazes
///
/// It works like this:
///
/// 1. Begins with an original grid as a working field
///
/// 2. Bisects the field either horizontally or vertically by carving a passage
/// through the wall from a random cell
///
/// 3. Repeats step #2 with the areas on either side of the wall where the passage
/// was carved.
///
/// 4. Continues, recursively, until the maze reaches the desired resolution
impl Algorithm for RecursiveDivision {
    fn generate(&mut self, grid: &mut Grid) {
        let width = grid.width();
        let height = grid.height();
        RecursiveDivision::divide(grid, 0, 0, width - 1, height - 1);
    }
}

fn choose_orientation(width: usize, height: usize) -> Orientation {
    if width < height {
        return Orientation::Horizontal;
    }

    if height < width {
        return Orientation::Vertical;
    }

    let mut rng = thread_rng();
    if !rng.gen::<bool>() {
        Orientation::Horizontal
    } else {
        Orientation::Vertical
    }
}
