use super::Algorithm;
use crate::maze::grid::{Grid, cell::Cell};
use crate::utils::types::Coords;
use rand::prelude::*;

/// The "Recursive Backtracking" algorithm for generating mazes
///
/// This algorithm quite effectively creates narrow passages with multiple dead-ends which makes it
/// easy to get lost, thus eventually making it hard to solve the maze.
///
/// In most cases, this algorithm is fast. However, due to its recursive nature, it requires stack
/// space proportional to the longest acyclic path, which is, in the worst case, the entire maze. So
/// for exceptionally large mazes this algorithm can be fairly inefficient.
pub struct RecursiveBacktracking;

/// An implementation of the "Recursive Backtracking" algorithm for generating mazes
///
/// Here is how it works:
///
/// 1. Chooses a starting point in the field.
///
/// 2. Randomly chooses a wall at that point and carves a passage through to the adjacent cell,
///    but only if the adjacent cell has not been visited yet. This becomes the new current cell.
///
/// 3. If all adjacent cells have been visited, backs up to the last cell that has uncarved walls
///    and repeats.
///
/// 4. The algorithm ends when the process has backed all the way up to the starting
///    point.
impl Algorithm for RecursiveBacktracking {
    fn generate(&mut self, grid: &mut Grid) {
        let start_coords = (0, 0);
        carve_passages_from(start_coords, grid);
    }
}

fn carve_passages_from(coords: Coords, grid: &mut Grid) {
    let mut dirs = [Cell::NORTH, Cell::SOUTH, Cell::WEST, Cell::EAST];
    dirs.shuffle(&mut rand::rng());

    for dir in dirs {
        let next = match grid.get_next_cell_coords(coords, dir) {
            Ok(next) => next,
            Err(_) => continue,
        };

        if grid.is_cell_visited(next) {
            continue;
        }

        if let Ok(next) = grid.carve_passage(coords, dir) {
            carve_passages_from(next, grid);
        }
    }
}
