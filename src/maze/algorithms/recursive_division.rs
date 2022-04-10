use super::Algorithm;
use crate::maze::grid::{pole::Pole, Grid};
use rand::prelude::*;

enum Orientation {
    Horizontal,
    Vertical,
}

/// The "Recursive Division" algorithm for generating mazes
///
/// This is one, and the only one in this library so far, that uses a "wall adding" technique
/// instead of a "passage carving". It's also novel in its fractal nature: you could theoretically
/// continue the process indefinitely at progressively finer and finer levels of detail on demand.
/// This may be useful when developing a game where a player wanders from one section of the maze
/// to another without a need to store the entire maze in the memory.
///
/// And yet, the algorithm's fractal nature leads to some visual artifacts and bottlenecks like
/// a single passage between two sections that effectevily divide the entire mize into two distinct
/// regions, thus making it easy to spot the passage and work backward to a solution.
pub struct RecursiveDivision;

impl RecursiveDivision {
    fn divide(
        &mut self,
        grid: &mut Grid,
        x: usize,
        y: usize,
        width: usize,
        height: usize,
        orientation: Orientation,
    ) {
        if width < 2 || height < 2 {
            return;
        }

        let mut rng = rand::thread_rng();

        // Get X and Y coordinates of a cell where walls will be drawn from, i.e. determine a vector
        // start coordinates
        let mut wx = x + match orientation {
            Orientation::Vertical if width > 2 => rng.gen_range(0..width - 2),
            _ => 0,
        };

        let mut wy = y + match orientation {
            Orientation::Horizontal if height > 2 => rng.gen_range(0..height - 2),
            _ => 0,
        };

        // Get X and Y coordinates of a cell where a passage will be carved through the wall
        let px = wx
            + match orientation {
                Orientation::Horizontal => rng.gen_range(0..width),
                Orientation::Vertical => 0,
            };

        let py = wy
            + match orientation {
                Orientation::Horizontal => 0,
                Orientation::Vertical => rng.gen_range(0..height),
            };

        // Get X and Y coordinates of a vector direction in which walls will be added
        let dx = match orientation {
            Orientation::Horizontal => 1,
            Orientation::Vertical => 0,
        };

        let dy = match orientation {
            Orientation::Horizontal => 0,
            Orientation::Vertical => 1,
        };

        // Determine how long the entire wall will be
        let length = match orientation {
            Orientation::Horizontal => width,
            Orientation::Vertical => height,
        };

        // Define what direction is corresponding to the wall orientation
        let dir = match orientation {
            Orientation::Horizontal => Pole::S,
            Orientation::Vertical => Pole::E,
        };

        // Add the walls and carve a passage
        for _ in 0..length {
            if wx != px || wy != py {
                grid.add_wall((wx, wy), dir)
            }

            wx += dx;
            wy += dy;
        }

        // Determine the bounds of the I-st sub-grid and get it split
        let nx = x;
        let ny = y;

        let (w, h) = match orientation {
            Orientation::Horizontal => (width, wy - y + 1),
            Orientation::Vertical => (wx - x + 1, height),
        };

        self.divide(grid, nx, ny, w, h, choose_orientation(w, h));

        // Determine the bounds of the II-nd sub-grid and get it split
        let (nx, ny) = match orientation {
            Orientation::Horizontal => (x, wy + 1),
            Orientation::Vertical => (wx + 1, y),
        };

        let (w, h) = match orientation {
            Orientation::Horizontal => (width, y + height - wy - 1),
            Orientation::Vertical => (x + width - wx - 1, height),
        };

        self.divide(grid, nx, ny, w, h, choose_orientation(w, h));
    }
}

/// An implementation of the "Recursive Division" algorithm for generating mazes
///
/// It works like this:
///
/// 1. Begins with an empty field
///
/// 2. Bisects the field with a wall, either horizontally or vertically. Adds a single passage
/// through the wall
///
/// 3. Repeats step #2 with the areas on either side of the wall
///
/// 4. Continues, recursively, until the maze reaches the desired resolution
impl Algorithm for RecursiveDivision {
    fn generate(&mut self, grid: &mut Grid) {
        // Recursive division algorithm acts as a wall adder, i.e. uses a "wall adding" technique,
        // rather than a "passage carving" one. To start adding walls to the maze cells we "drain"
        // the grid, thus having a proper input for this algorithm
        grid.drain();

        let width = grid.width();
        let height = grid.height();

        self.divide(grid, 0, 0, width, height, choose_orientation(width, height))
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
    if rng.gen::<bool>() == false {
        Orientation::Horizontal
    } else {
        Orientation::Vertical
    }
}
