use std::fmt;

use bevy::{ecs::component::Component, reflect::Reflect};
use bitflags::bitflags;

bitflags! {
    /// Maze Cell defining open passages
    #[derive(Default, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Component, Reflect)]
    #[reflect(opaque)]
    pub struct Cell: u8 {
        /// Has passage to NORTH
        const NORTH = 0b0001;
        /// Has passage to SOUTH
        const SOUTH = 0b0010;
        /// Has passage to EAST
        const EAST =  0b0100;
        /// Has passage to WEST
        const WEST =  0b1000;
    }
}

impl Cell {
    /// Returns bit string representation
    pub fn to_bits_str(&self) -> String {
        format!("{:0>4b}", self.bits())
    }
}

impl fmt::Display for Cell {
    /// Writes a formatted maze into a buffer
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let names = self
            .iter_names()
            .map(|(name, _cell)| &name[0..1])
            .collect::<Vec<_>>();
        let names = if names.is_empty() {
            "BLOCKED".to_string()
        } else {
            names.join("")
        };
        write!(f, "{}", names)?;
        Ok(())
    }
}

#[derive(Default, Debug, Copy, Clone, PartialEq, Eq)]
pub struct CellStatus {
    visited: bool,
    marked: bool,
}

impl CellStatus {
    pub const fn visited(&self) -> bool {
        self.visited
    }

    pub const fn marked(&self) -> bool {
        self.marked
    }

    pub fn visit(&mut self) {
        self.visited = true;
    }

    pub fn mark(&mut self) {
        self.marked = true;
    }
}
#[cfg(test)]
mod tests {
    use super::Cell;

    #[test]
    fn empty_is_0000() {
        let zero = Cell::empty();

        assert_eq!(zero.to_bits_str(), "0000")
    }

    #[test]
    fn all_is_1111() {
        let zero = Cell::all();

        assert_eq!(zero.to_bits_str(), "1111")
    }

    #[test]
    fn north_is_0001() {
        let zero = Cell::NORTH;

        assert_eq!(zero.to_bits_str(), "0001")
    }

    #[test]
    fn south_is_0010() {
        let zero = Cell::SOUTH;

        assert_eq!(zero.to_bits_str(), "0010")
    }

    #[test]
    fn east_is_0100() {
        let zero = Cell::EAST;

        assert_eq!(zero.to_bits_str(), "0100")
    }

    #[test]
    fn west_is_1000() {
        let zero = Cell::WEST;

        assert_eq!(zero.to_bits_str(), "1000")
    }

    #[test]
    fn empty_cell_name() {
        let cell = Cell::empty();

        assert_eq!(cell.to_string(), "BLOCKED");
    }

    #[test]
    fn all_cell_name() {
        let cell = Cell::all();

        assert_eq!(cell.to_string(), "NSEW");
    }

    #[test]
    fn sw_cell_name() {
        let cell = Cell::SOUTH | Cell::WEST;

        assert_eq!(cell.to_string(), "SW");
    }
}
