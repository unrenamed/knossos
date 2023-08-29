pub mod cell;
use self::cell::CellStatus;

use super::errors::TransitError;
use crate::utils::types::Coords;
use cell::Cell;
use std::{fmt};

type TransitResult<T> = Result<T, TransitError>;

pub struct Grid {
    width: usize,
    height: usize,
    cells: Vec<Cell>,
    cell_statuses: Vec<CellStatus>,
}

impl Grid {
    pub fn new(width: usize, height: usize) -> Grid {
        Grid {
            width,
            height,
            cells: vec![Cell::default(); width * height],
            cell_statuses: vec![CellStatus::default(); width * height],
        }
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn mark_cell(&mut self, coords: Coords) {
        self.get_cell_status_mut(coords).mark()
    }

    pub fn is_cell_visited(&self, coords: Coords) -> bool {
        self.get_cell_status(coords).visited()
    }

    pub fn is_cell_marked(&self, coords: Coords) -> bool {
        self.get_cell_status(coords).marked()
    }

    pub fn get_cell_status(&self, coords: Coords) -> CellStatus {
        let (x, y) = coords;
        self.cell_statuses[y * self.width + x]
    }

    pub fn is_carved(&self, coords: Coords, direction: Cell) -> bool {
        let (x, y) = coords;
        self.cells[y * self.width + x].contains(direction)
    }

    pub fn carve_passage(&mut self, coords: Coords, direction: Cell) -> TransitResult<Coords> {
        let (x, y) = coords;
        let (nx, ny) = self.get_next_cell_coords(coords, direction)?;

        match direction {
            Cell::NORTH => {
                self.cells[y * self.width + x] |= Cell::NORTH;
                self.cells[ny * self.width + nx] |= Cell::SOUTH;
            }
            Cell::SOUTH => {
                self.cells[y * self.width + x] |= Cell::SOUTH;
                self.cells[ny * self.width + nx] |= Cell::NORTH;
            }
            Cell::EAST => {
                self.cells[y * self.width + x] |= Cell::EAST;
                self.cells[ny * self.width + nx] |= Cell::WEST;
            }
            Cell::WEST => {
                self.cells[y * self.width + x] |= Cell::WEST;
                self.cells[ny * self.width + nx] |= Cell::EAST;
            }
            _ => (),
        }

        self.visit_cell(coords);
        self.visit_cell((nx, ny));

        Ok((nx, ny))
    }

    pub fn get_next_cell_coords(
        &self,
        coords: Coords,
        direction: Cell,
    ) -> TransitResult<Coords> {
        self.validate_transit(coords, direction)?;

        let (x, y) = coords;
        let (nx, ny) = match direction {
            Cell::NORTH => (x, y - 1),
            Cell::SOUTH => (x, y + 1),
            Cell::WEST => (x - 1, y),
            Cell::EAST => (x + 1, y),
            _ => (x, y),
        };
        Ok((nx, ny))
    }

    fn visit_cell(&mut self, coords: Coords) {
        self.get_cell_status_mut(coords).visit()
    }

    fn get_cell_status_mut(&mut self, coords: Coords) -> &mut CellStatus {
        let (x, y) = coords;
        &mut self.cell_statuses[y * self.width + x]
    }

    fn validate_transit(&self, coords: Coords, direction: Cell) -> TransitResult<()> {
        let (x, y) = coords;
        let reason = match direction {
            Cell::NORTH if y < 1 => Some("First row in the grid cannot go North"),
            Cell::SOUTH if y + 1 == self.height => Some("Last row in the grid cannot go South"),
            Cell::WEST if x < 1 => Some("First cell in a row cannot go West"),
            Cell::EAST if x + 1 == self.width => Some("Last column in the grid cannot go East"),
            _ => None,
        };

        if reason.is_none() {
            return Ok(());
        }

        Err(TransitError {
            coords: (x, y),
            reason: reason.unwrap().to_string(),
        })
    }
}

impl fmt::Display for Grid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let top_border = "_".repeat(self.width * 2 - 1);

        writeln!(f, " {} ", top_border)?;

        for y in 0..self.height {
            write!(f, "|")?; // display left border

            for x in 0..self.width {
                if self.is_carved((x, y), Cell::SOUTH) {
                    write!(f, " ")?;
                } else {
                    write!(f, "_")?;
                }

                if self.is_carved((x, y), Cell::EAST) {
                    if self.is_carved((x, y), Cell::SOUTH)
                        || self.is_carved((x + 1, y), Cell::SOUTH)
                    {
                        write!(f, " ")?;
                    } else {
                        write!(f, "_")?;
                    }
                } else {
                    write!(f, "|")?;
                }
            }

            writeln!(f)?; // goto next line
        }

        Ok(())
    }
}
