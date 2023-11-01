use super::types::Coords;

pub struct RandPositions;

impl RandPositions {
    #[cfg(not(test))]
    #[cfg(not(tarpaulin_include))]
    pub fn rand(positions: &mut Vec<Coords>) -> &mut Vec<Coords> {
        use rand::seq::SliceRandom;
        let mut rng = rand::thread_rng();
        positions.shuffle(&mut rng);
        positions
    }

    #[cfg(test)]
    #[cfg(not(tarpaulin_include))]
    pub fn rand(positions: &mut Vec<Coords>) -> &mut Vec<Coords> {
        positions
    }
}
