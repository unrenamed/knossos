use super::pole::Pole;
use super::walls::Walls;

#[derive(Debug, Clone)]
pub struct Cell {
    walls: Walls,
    visited: bool,
    marked: bool,
}

impl Cell {
    pub fn new() -> Cell {
        Cell {
            walls: Walls::init(),
            visited: false,
            marked: false,
        }
    }

    pub fn empty() -> Cell {
        Cell {
            walls: Walls::empty(),
            visited: false,
            marked: false,
        }
    }

    pub fn get_walls(&self) -> &Walls {
        &self.walls
    }

    pub fn visited(&self) -> bool {
        self.visited
    }

    pub fn marked(&self) -> bool {
        self.marked
    }

    pub fn add_wall(&mut self, pole: Pole) {
        self.walls.add(pole)
    }

    pub fn remove_wall(&mut self, pole: Pole) {
        self.walls.remove(pole);
    }

    pub fn visit(&mut self) {
        self.visited = true;
    }

    pub fn mark(&mut self) {
        self.marked = true;
    }
}
