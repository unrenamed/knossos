use super::Algorithm;
use crate::{
    maze::grid::{Grid, cell::Cell},
    utils::types::Coords,
};
use rand::prelude::*;

/// The Aldous-Broder's algorithm for generating mazes.
///
/// This is an easy one to implement. And yet, it is also one of the least intelligent algorithms,
/// since the latest steps may take so much time that you may not want to wait until it's finished.
/// It is not even guaranteed to finish if you get really unlucky with the random.
pub struct AldousBroder;

impl AldousBroder {}

/// An implementation of Aldous-Broder's algorithm for generating mazes.
///
/// The problem domain the algorithm was created for is finding unform spanning trees. Here is how
/// it works:
///
/// 1. Chooses any vertex.
///
/// 2. Chooses a connected neighbor of the vertex and travels to it. If the neighbor has not yet
///    been visited, adds the traveled edge to the spanning tree.
///
/// 3. Repeats step 2 until all vertices have been visited.
impl Algorithm for AldousBroder {
    fn generate(&mut self, grid: &mut Grid, start_coords: Option<Coords>) {
        let (mut x, mut y) = start_coords.unwrap_or_else(|| get_start_coords(grid));

        let mut remaining = grid.width() * grid.height() - 1; // the number of remaining unvisited cells

        while remaining > 0 {
            let mut directions = [Cell::NORTH, Cell::SOUTH, Cell::WEST, Cell::EAST];
            directions.shuffle(&mut rand::rng());

            for dir in directions {
                let next_cell = grid.get_next_cell_coords((x, y), dir);
                if next_cell.is_err() {
                    continue;
                }

                let (nx, ny) = next_cell.unwrap();
                if !grid.is_cell_visited((nx, ny)) {
                    grid.carve_passage((x, y), dir).unwrap();
                    remaining -= 1;
                }

                x = nx;
                y = ny;
                break;
            }
        }
    }

    fn has_start_coords(&self) -> bool {
        true
    }

    fn name(&self) -> &'static str {
        "AldousBroder"
    }
}

fn get_start_coords(grid: &Grid) -> Coords {
    let mut rng = rand::rng();
    let y = rng.random_range(0..grid.height());
    let x = rng.random_range(0..grid.width());
    (x, y)
}
