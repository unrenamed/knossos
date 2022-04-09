use lazy_static::lazy_static;
use std::collections::HashMap;
use std::hash::Hash;

#[derive(Debug, Eq, PartialEq, Hash, Clone, Copy)]
pub enum Pole {
    N,
    S,
    E,
    W,
}

lazy_static! {
    pub static ref OPPOSITE_POLES: HashMap<Pole, Pole> = {
        let mut map = HashMap::with_capacity(4);
        map.insert(Pole::N, Pole::S);
        map.insert(Pole::S, Pole::N);
        map.insert(Pole::E, Pole::W);
        map.insert(Pole::W, Pole::E);
        map
    };
    pub static ref POLE_DIR_X: HashMap<Pole, i8> = {
        let mut map = HashMap::with_capacity(4);
        map.insert(Pole::N, 0);
        map.insert(Pole::S, 0);
        map.insert(Pole::E, 1);
        map.insert(Pole::W, -1);
        map
    };
    pub static ref POLE_DIR_Y: HashMap<Pole, i8> = {
        let mut map = HashMap::with_capacity(4);
        map.insert(Pole::N, -1);
        map.insert(Pole::S, 1);
        map.insert(Pole::E, 0);
        map.insert(Pole::W, 0);
        map
    };
}
