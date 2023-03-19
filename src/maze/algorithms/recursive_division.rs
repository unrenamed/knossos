use super::Algorithm;
use crate::maze::grid::{cell::Cell, Grid};
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
    fn divide(&mut self, grid: &mut Grid, x: usize, y: usize, ax: usize, ay: usize) {
        let w = ax - x;
        let h = ay - y;

        if w < 2 || h < 2 {
            if w > 1 {
                for cx in x..(ax - 1) {
                    grid.carve_passage((cx, y), Cell::EAST).unwrap();
                }
            } else if h > 1 {
                for cy in y..(ay - 1) {
                    grid.carve_passage((x, cy), Cell::SOUTH).unwrap();
                }
            }
            return;
        }

        let mut rng = rand::thread_rng();

        // Which way a field with the given dimensions ought to be bisected
        let orientation = choose_orientation(w, h);

        // Get X and Y coordinates of a cell where a wall will be carved
        let px = rng.gen_range(
            x..match orientation {
                Orientation::Vertical => ax - 1,
                Orientation::Horizontal => ax,
            },
        );

        let py = rng.gen_range(
            y..match orientation {
                Orientation::Vertical => ay,
                Orientation::Horizontal => ay - 1,
            },
        );

        // Define what direction is corresponding to the wall orientation
        let dir = match orientation {
            Orientation::Horizontal => Cell::SOUTH,
            Orientation::Vertical => Cell::EAST,
        };

        // Carve passage
        let (nx, ny) = grid.carve_passage((px, py), dir).unwrap();

        // Determine the bounds of the sub-fields and get them split
        match orientation {
            Orientation::Horizontal => {
                // I sub-field
                self.divide(grid, x, y, ax, ny);
                // II sub-field
                self.divide(grid, x, ny, ax, ay);
            }
            Orientation::Vertical => {
                // I sub-field
                self.divide(grid, x, y, nx, ay);
                // II sub-field
                self.divide(grid, nx, y, ax, ay);
            }
        }
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
        let width = grid.width();
        let height = grid.height();
        self.divide(grid, 0, 0, width, height);
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
