use super::Algorithm;
use crate::utils::types::Coords;
use crate::maze::grid::{Grid, cell::Cell};

use rand::prelude::*;

/// The "Hunt & Kill" algorithm for generating mazes
///
/// This is similar to the recursive backtracker: they both tend to generate long, winding passages
/// with fewer dead-ends than most of the other algorithms. However, this one differs in that it
/// will search the grid iteratively, looking for a new blank cell when it encounters a dead-end.
pub struct HuntAndKill {
    hunt_start_index: usize,
}

impl HuntAndKill {
    /// Create a new instance of the algorithm with a default settings for the "hunt" phase
    pub const fn new() -> HuntAndKill {
        HuntAndKill {
            hunt_start_index: 0,
        }
    }

    fn walk<R: Rng>(&self, coords: Coords, grid: &mut Grid, rng: &mut R) -> Option<Coords> {
        let mut directions = [Cell::NORTH, Cell::SOUTH, Cell::WEST, Cell::EAST];
        directions.shuffle(rng);

        for dir in directions {
            if let Ok(next_coords) = grid.get_next_cell_coords(coords, dir) {
                if !grid.is_cell_visited(next_coords) {
                    return grid.carve_passage(coords, dir).ok();
                }
            }
        }

        None
    }

    fn hunt(&mut self, grid: &mut Grid) -> Option<Coords> {
        let directions = [Cell::NORTH, Cell::SOUTH, Cell::WEST, Cell::EAST];

        for y in self.hunt_start_index..grid.height() {
            let mut unvisited_cells_count = 0;

            for x in 0..grid.width() {
                if grid.is_cell_visited((x, y)) {
                    continue;
                } else {
                    unvisited_cells_count += 1;
                }

                for dir in directions {
                    if let Ok(next_coords) = grid.get_next_cell_coords((x, y), dir) {
                        if grid.is_cell_visited(next_coords) {
                            grid.carve_passage((x, y), dir).ok();
                            return Some((x, y));
                        }
                    }
                }
            }

            if unvisited_cells_count == 0 {
                self.hunt_start_index = y + 1;
            }
        }
        None
    }
}

impl Default for HuntAndKill {
    fn default() -> Self {
        Self::new()
    }
}

/// An implementation of the "Hunt & Kill" algorithm for generating mazes
///
/// The algorithm consists of two main phases: walk and hunt, which repeat from row to row.
///
/// Here is how it works in detail:
///
/// 1. Chooses a starting location
///
/// 2. Performs a random walk, carving passages to unvisited neighbors, until the current cell has
///    no unvisited neighbors.
///
/// 3. Enters the “hunt” mode, where you scan the grid looking for an
///    unvisited cell that is adjacent to a visited cell. If found, carves a passage between the two
///    and lets the formerly unvisited cell be the new starting location.
///
/// 4. Repeats steps 2 and 3 until the "hunt" mode scans the entire grid and finds no unvisited
///    cells.
///
/// # Optimization
///
/// It is worth mentioning that unlike the standard version of this algorithm which gets a little
/// slow towards the end, where the "hunt" phase has to search over nearly the entire grid to find a
/// candidate cell, this implementation has a simple optimization that speeds up the later stages of
/// the algorithm. Thus, this algorithm is still pretty fast
impl Algorithm for HuntAndKill {
    fn generate(&mut self, grid: &mut Grid, rng: &mut StdRng) {
        let start_coords = get_start_coords(grid, rng);
        let mut x = start_coords.0;
        let mut y = start_coords.1;

        loop {
            if let Some((nx, ny)) = self.walk((x, y), grid, rng) {
                x = nx;
                y = ny;
            } else if let Some((nx, ny)) = self.hunt(grid) {
                x = nx;
                y = ny;
            } else {
                break;
            }
        }
    }
}

fn get_start_coords<R: Rng>(grid: &Grid, rng: &mut R) -> Coords {
    let y = rng.random_range(0..grid.height());
    let x = rng.random_range(0..grid.width());
    (x, y)
}
