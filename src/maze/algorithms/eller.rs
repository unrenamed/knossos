use super::{Algorithm, BOOL_TRUE_PROBABILITY};
use crate::utils::types::Coords;
use crate::maze::grid::{Grid, cell::Cell as GridCell};

use rand::prelude::*;
use std::{cell::RefCell, collections::HashMap};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct CellId(usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct SetId(usize);

#[derive(Debug, Clone, Copy)]
struct Cell {
    _id: CellId,
    set_id: SetId,
    coords: Coords,
}

struct State {
    width: usize,
    next_set_id: Option<usize>,
    row_num: usize,
    cells: HashMap<CellId, RefCell<Cell>>,
}

impl State {
    fn new(row_num: usize, next_set_id: Option<usize>, width: usize) -> State {
        State {
            width,
            next_set_id,
            row_num,
            cells: HashMap::new(),
        }
    }

    fn next(&self) -> State {
        State::new(self.row_num + 1, self.next_set_id, self.width)
    }

    fn populate(mut self) -> Self {
        for n in 1..=self.width {
            let cell_id = CellId(n);
            if self.cells.contains_key(&cell_id) {
                continue;
            }

            self.next_set_id = Some(self.next_set_id.unwrap_or(0) + 1);
            let set_id = SetId(self.next_set_id.unwrap());

            self.add(cell_id, set_id, (n - 1, self.row_num));
        }

        self
    }

    fn add(&mut self, _id: CellId, set_id: SetId, coords: Coords) {
        let cell = Cell {
            _id,
            set_id,
            coords,
        };
        self.cells.insert(_id, RefCell::new(cell));
    }

    fn connect(&mut self, sink_id: CellId, target_id: CellId) {
        let sink = self.cells.get(&sink_id).unwrap().borrow();
        let mut target = self.cells.get(&target_id).unwrap().borrow_mut();
        target.set_id = sink.set_id;
    }

    fn connected(&self, id: CellId, other_id: CellId) -> bool {
        let cell = self.cells.get(&id).unwrap().borrow();
        let other = self.cells.get(&other_id).unwrap().borrow();
        cell.set_id == other.set_id
    }

    fn get_cell_coords(&self, id: CellId) -> Coords {
        let cell = self.cells.get(&id).unwrap().borrow();
        cell.coords
    }

    fn sets(&self) -> HashMap<SetId, Vec<CellId>> {
        let mut sets: HashMap<SetId, Vec<CellId>> = HashMap::new();

        self.cells.iter().for_each(|(id, cell)| {
            let cell = cell.borrow();

            if let Some(cells) = sets.get_mut(&cell.set_id) {
                (*cells).push(*id);
            } else {
                sets.insert(cell.set_id, vec![*id]);
            }
        });

        sets
    }
}

/// The Eller's algorithm for generating mazes
///
/// This is one of the best algorithms in terms of space complexity since it
/// runs over a single row at a time. Moreover, by making a small change,
/// this one can generate mazes of infinite size in linear time.
pub struct Eller;

impl Eller {
    /// Randomly joins adjacent cells, but only if they are not in the same set
    fn connect_disjoint_sets<R: Rng>(
        &self,
        state: &mut State,
        grid: &mut Grid,
        is_last_row: bool,
        rng: &mut R,
    ) {
        for c in 1..state.width {
            let cell_id = CellId(c);
            let next_cell_id = CellId(c + 1);

            if state.connected(cell_id, next_cell_id)
                || (!is_last_row && rng.random_bool(BOOL_TRUE_PROBABILITY))
            {
                continue;
            }

            state.connect(cell_id, next_cell_id);
            let (x, y) = state.get_cell_coords(cell_id);
            grid.carve_passage((x, y), GridCell::EAST).unwrap();
        }
    }

    /// For each set, creates at least one vertical connection downward to the next row
    fn add_vertical_connections<R: Rng>(
        &self,
        state: &mut State,
        grid: &mut Grid,
        is_last_row: bool,
        rng: &mut R,
    ) -> State {
        let mut next_state = state.next();

        if is_last_row {
            return next_state.populate();
        }

        for (set_id, cells) in state.sets() {
            for cell_id in self.cells_to_connect(cells, rng) {
                let (x, y) = state.get_cell_coords(cell_id);
                grid.carve_passage((x, y), GridCell::SOUTH).unwrap();
                next_state.add(cell_id, set_id, (x, y + 1));
            }
        }

        next_state.populate()
    }

    /// Selects random cells to carve vertical passages from
    fn cells_to_connect<R: Rng>(&self, cells: Vec<CellId>, rng: &mut R) -> Vec<CellId> {
        let mut cells = cells;
        cells.shuffle(rng);

        let connect_count = if cells.len() >= 2 {
            rng.random_range(1..cells.len())
        } else {
            1
        };

        cells
            .iter()
            .take(connect_count)
            .cloned()
            .collect::<Vec<CellId>>()
    }
}

/// An implementation of Eller's algorithm for generating mazes
///
/// Generates maze following the algorithm below:
///
/// 1. Initializes the cells of the first row to each exist in their own set.
///
/// 2. Randomly joins adjacent cells, but only if they are not in the same set. When joining
///    adjacent cells, merges the cells of both sets into a single set, indicating that all cells in
///    both sets are now connected.
///
/// 3. For each set, randomly creates vertical connections downward to the next row. Each
///    remaining set must have at least one vertical connection. The cells in the next row thus
///    connected must share the set of the cell above them.
///
/// 4. Fleshes out the next row by putting any remaining cells into their own sets.
///
/// 5. Repeats until the last row is reached.
///
/// 6. For the last row, joins all adjacent cells that do not share a set, and omit the vertical
///    connections.
impl Algorithm for Eller {
    fn generate(&mut self, grid: &mut Grid, rng: &mut StdRng) {
        let mut state = State::new(0, None, grid.width()).populate();

        for row in 0..grid.height() {
            let is_last_row = row == grid.height() - 1;
            self.connect_disjoint_sets(&mut state, grid, is_last_row, rng);
            state = self.add_vertical_connections(&mut state, grid, is_last_row, rng);
        }
    }
}
