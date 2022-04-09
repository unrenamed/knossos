use super::pole::Pole;
use std::collections::HashMap;

#[derive(Debug, Clone)]
// A tuple struct that describes a cell walls which are stored in a HashMap<Pole, bool>, where:
// - Pole is a wall position
// - bool indicates if a wall exists or not. True by default
pub struct Walls(HashMap<Pole, bool>);

impl Walls {
    pub fn init() -> Walls {
        Walls::build(true)
    }

    pub fn empty() -> Walls {
        Walls::build(false)
    }

    pub fn add(&mut self, pole: Pole) {
        let exist = self.get_wall_mut(pole);
        *exist = true;
    }

    pub fn remove(&mut self, pole: Pole) {
        let exist = self.get_wall_mut(pole);
        *exist = false;
    }

    pub fn carved(&self, pole: Pole) -> bool {
        let exist = self.get_wall(pole);
        *exist == false
    }

    fn build(is_exist: bool) -> Walls {
        let mut walls = HashMap::with_capacity(4);
        walls.insert(Pole::N, is_exist);
        walls.insert(Pole::S, is_exist);
        walls.insert(Pole::E, is_exist);
        walls.insert(Pole::W, is_exist);
        Walls(walls)
    }

    fn get_wall(&self, pole: Pole) -> &bool {
        self.0.get(&pole).unwrap()
    }

    fn get_wall_mut(&mut self, pole: Pole) -> &mut bool {
        self.0.get_mut(&pole).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn build() {
        let walls = Walls::init();
        assert_eq!(walls.get_wall(Pole::N), &true);
        assert_eq!(walls.get_wall(Pole::W), &true);
        assert_eq!(walls.get_wall(Pole::S), &true);
        assert_eq!(walls.get_wall(Pole::E), &true);
    }

    #[test]
    fn build_empty() {
        let walls = Walls::empty();
        assert_eq!(walls.get_wall(Pole::N), &false);
        assert_eq!(walls.get_wall(Pole::E), &false);
        assert_eq!(walls.get_wall(Pole::W), &false);
        assert_eq!(walls.get_wall(Pole::S), &false);
    }

    #[test]
    fn add_wall() {
        let mut walls = Walls::empty();
        walls.add(Pole::N);
        assert_eq!(walls.get_wall(Pole::N), &true);
        assert_eq!(walls.get_wall(Pole::E), &false);
        assert_eq!(walls.get_wall(Pole::W), &false);
        assert_eq!(walls.get_wall(Pole::S), &false);
    }

    #[test]
    fn remove_wall() {
        let mut walls = Walls::init();
        walls.remove(Pole::N);
        assert_eq!(walls.get_wall(Pole::N), &false);
        assert_eq!(walls.get_wall(Pole::E), &true);
        assert_eq!(walls.get_wall(Pole::W), &true);
        assert_eq!(walls.get_wall(Pole::S), &true);
    }

    #[test]
    fn is_wall_carved() {
        let mut walls = Walls::init();
        walls.remove(Pole::N);
        assert_eq!(walls.carved(Pole::N), true);
    }
}
