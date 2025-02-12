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
    /// Returns bits &str representation.
    ///  > use `to_bits_string` for string value
    pub fn to_bits_str(&self) -> &'static str {
        let bits = format!("{:0>4b}", self.bits());
        bits.leak()
    }

    /// Returns bits string representation.
    pub fn to_bits_string(&self) -> String {
        format!("{:0>4b}", self.bits())
    }

    /// Returns bits u8 representation.
    /// > analogous to `bits`.
    pub const fn to_bits(&self) -> u8 {
        self.bits()
    }

    /// Amount of walls present: [0..=4].
    pub const fn walls_count(&self) -> u8 {
        self.bits().count_zeros().saturating_sub(4) as u8
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
    fn empty_is_0000_str() {
        let zero = Cell::empty();

        assert_eq!(zero.to_bits_str(), "0000")
    }

    #[test]
    fn all_is_1111_str() {
        let zero = Cell::all();

        assert_eq!(zero.to_bits_str(), "1111")
    }

    #[test]
    fn north_is_0001_str() {
        let zero = Cell::NORTH;

        assert_eq!(zero.to_bits_str(), "0001")
    }

    #[test]
    fn south_is_0010_str() {
        let zero = Cell::SOUTH;

        assert_eq!(zero.to_bits_str(), "0010")
    }

    #[test]
    fn east_is_0100_str() {
        let zero = Cell::EAST;

        assert_eq!(zero.to_bits_str(), "0100")
    }

    #[test]
    fn west_is_1000_str() {
        let zero = Cell::WEST;

        assert_eq!(zero.to_bits_str(), "1000")
    }

    #[test]
    fn empty_is_0000_bits() {
        let zero = Cell::empty();

        assert_eq!(zero.to_bits(), 0b0000)
    }

    #[test]
    fn all_is_1111_bits() {
        let zero = Cell::all();

        assert_eq!(zero.to_bits(), 0b1111)
    }

    #[test]
    fn north_is_0001_bits() {
        let zero = Cell::NORTH;

        assert_eq!(zero.to_bits(), 0b0001)
    }

    #[test]
    fn south_is_0010_bits() {
        let zero = Cell::SOUTH;

        assert_eq!(zero.to_bits(), 0b0010)
    }

    #[test]
    fn east_is_0100_bits() {
        let zero = Cell::EAST;

        assert_eq!(zero.to_bits(), 0b0100)
    }

    #[test]
    fn west_is_1000_bits() {
        let zero = Cell::WEST;

        assert_eq!(zero.to_bits(), 0b1000)
    }

    #[test]
    fn empty_is_0000() {
        let zero = Cell::empty();

        assert_eq!(zero.to_bits_string(), "0000")
    }

    #[test]
    fn all_is_1111() {
        let zero = Cell::all();

        assert_eq!(zero.to_bits_string(), "1111")
    }

    #[test]
    fn north_is_0001() {
        let zero = Cell::NORTH;

        assert_eq!(zero.to_bits_string(), "0001")
    }

    #[test]
    fn south_is_0010() {
        let zero = Cell::SOUTH;

        assert_eq!(zero.to_bits_string(), "0010")
    }

    #[test]
    fn east_is_0100() {
        let zero = Cell::EAST;

        assert_eq!(zero.to_bits_string(), "0100")
    }

    #[test]
    fn west_is_1000() {
        let zero = Cell::WEST;

        assert_eq!(zero.to_bits_string(), "1000")
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

    #[test]
    fn get_all_walls_count() {
        assert_eq!(Cell::empty().walls_count(), 4);
        assert_eq!(Cell::SOUTH.walls_count(), 3);
        assert_eq!(Cell::NORTH.walls_count(), 3);
        assert_eq!(Cell::WEST.walls_count(), 3);
        assert_eq!(Cell::EAST.walls_count(), 3);
        assert_eq!((Cell::SOUTH | Cell::NORTH).walls_count(), 2);
        assert_eq!((Cell::EAST | Cell::WEST).walls_count(), 2);
        assert_eq!((Cell::SOUTH | Cell::NORTH | Cell::WEST).walls_count(), 1);
        assert_eq!((Cell::EAST | Cell::NORTH | Cell::WEST).walls_count(), 1);
        assert_eq!(Cell::all().walls_count(), 0);
    }
}
