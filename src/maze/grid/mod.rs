pub mod cell;
pub mod pole;
pub mod walls;

use super::errors::TransitError;
use crate::utils::{num, types::Coords};
use cell::Cell;
use pole::{Pole, OPPOSITE_POLES, POLE_DIR_X, POLE_DIR_Y};
use std::fmt;
use std::iter;

type Cells = Vec<Vec<Cell>>;
type TransitResult<T> = Result<T, TransitError>;

pub struct Grid {
    width: usize,
    height: usize,
    cells: Cells,
}

impl Grid {
    pub fn new(width: usize, height: usize) -> Grid {
        Grid {
            width,
            height,
            cells: vec![vec![Cell::new(); width]; height],
        }
    }

    pub fn cells(&self) -> &Cells {
        &self.cells
    }

    // Emptys the grid cells thus keeping the border walls only. This should be used when a maze
    // generation algorithm that uses a "wall adding" technique, rather than a "passage carving"
    // one, is selected
    pub fn drain(&mut self) {
        let mut cells = vec![vec![Cell::empty(); self.width]; self.height];

        // Add Northern walls to the cells in the first row
        for cell in cells[0].iter_mut() {
            (*cell).add_wall(Pole::N);
        }

        // Add Southern walls to the cells in the last row
        for cell in cells[self.height - 1].iter_mut() {
            (*cell).add_wall(Pole::S);
        }

        // Add Western walls to the cells in the first column
        for y in 0..cells.len() {
            for x in 0..cells[y].len() {
                if x == 0 {
                    cells[y][x].add_wall(Pole::W);
                }
            }
        }

        // Add Eastern walls to the cells in the last column
        for y in 0..cells.len() {
            for x in 0..cells[y].len() {
                if x == cells[y].len() - 1 {
                    cells[y][x].add_wall(Pole::E);
                }
            }
        }

        self.cells = cells;
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn mark_cell(&mut self, coords: Coords) {
        self.get_cell_mut(coords).mark()
    }

    pub fn is_cell_visited(&self, coords: Coords) -> bool {
        self.get_cell(coords).visited()
    }

    pub fn is_cell_marked(&self, coords: Coords) -> bool {
        self.get_cell(coords).marked()
    }

    pub fn get_cell(&self, coords: Coords) -> &Cell {
        let (x, y) = coords;
        &self.cells[y][x]
    }

    pub fn add_wall(&mut self, coords: Coords, pole: Pole) {
        let next = self.get_next_cell_coords(coords, pole).unwrap();
        let opp_pole = *OPPOSITE_POLES.get(&pole).unwrap();

        // add a wall towards a pole
        self.get_cell_mut(coords).add_wall(pole);

        // add a wall to a next cell towards an opposite pole
        self.get_cell_mut(next).add_wall(opp_pole);
    }

    pub fn carve_passage(&mut self, coords: Coords, pole: Pole) -> TransitResult<Coords> {
        let next = self.get_next_cell_coords(coords, pole)?;
        let opp_pole = *OPPOSITE_POLES.get(&pole).unwrap();

        self.get_cell_mut(coords).remove_wall(pole); // remove a wall towards a pole
        self.get_cell_mut(next).remove_wall(opp_pole); // remove a wall of a next cell towards an opposite pole

        self.visit_cell(coords);
        self.visit_cell(next);

        Ok(next)
    }

    pub fn get_next_cell_coords(&mut self, coords: Coords, pole: Pole) -> TransitResult<Coords> {
        self.validate_transit(coords, pole)?;

        let (x, y) = coords;
        let nx = num::add(x, *POLE_DIR_X.get(&pole).unwrap());
        let ny = num::add(y, *POLE_DIR_Y.get(&pole).unwrap());

        Ok((nx, ny))
    }

    fn visit_cell(&mut self, coords: Coords) {
        self.get_cell_mut(coords).visit()
    }

    fn get_cell_mut(&mut self, coords: Coords) -> &mut Cell {
        let (x, y) = coords;
        &mut self.cells[y][x]
    }

    fn validate_transit(&self, coords: Coords, pole: Pole) -> TransitResult<()> {
        let (x, y) = coords;

        if x < 1 && pole == Pole::W {
            return Err(TransitError {
                coords: (x, y),
                reason: String::from("First cell in a row cannot go West"),
            });
        }

        if y < 1 && pole == Pole::N {
            return Err(TransitError {
                coords: (x, y),
                reason: String::from("First row in the grid cannot go North"),
            });
        }

        if x + 1 == self.width && pole == Pole::E {
            return Err(TransitError {
                coords: (x, y),
                reason: String::from("Last column in the grid cannot go East"),
            });
        }

        if y + 1 == self.height && pole == Pole::S {
            return Err(TransitError {
                coords: (x, y),
                reason: String::from("Last row in the grid cannot go South"),
            });
        }

        Ok(())
    }
}

impl fmt::Display for Grid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let top_border = iter::repeat("_")
            .take(self.width * 2 - 1)
            .collect::<String>();

        writeln!(f, " {} ", top_border)?;

        for y in 0..self.height {
            write!(f, "|")?; // display left border

            for x in 0..self.width {
                let walls = self.get_cell((x, y)).get_walls();

                if walls.carved(Pole::S) {
                    write!(f, " ")?;
                } else {
                    write!(f, "_")?;
                }

                if walls.carved(Pole::E) {
                    let next_cell_walls = self.get_cell((x + 1, y)).get_walls();
                    if walls.carved(Pole::S) || next_cell_walls.carved(Pole::S) {
                        write!(f, " ")?;
                    } else {
                        write!(f, "_")?;
                    }
                } else {
                    write!(f, "|")?;
                }
            }

            writeln!(f, "")?; // goto next line
        }

        Ok(())
    }
}
