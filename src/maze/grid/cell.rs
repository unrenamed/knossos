use bitflags::bitflags;

bitflags! {
    #[derive(Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
    pub struct Cell: u8 {
        const NORTH = 0b0001;
        const SOUTH = 0b0010;
        const EAST =  0b0100;
        const WEST =  0b1000;
    }
}

#[derive(Default, Copy, Clone)]
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
