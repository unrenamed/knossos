use bevy::prelude::*;
use pathfinding::prelude::astar;
use crate::{
    utils::types::{Goal, Start},
    Cell, CoordsComponent,
};
use std::{
    // hash::{Hash, Hasher},
    collections::HashMap,
};
/// Associated cost to the path [`Cell`]. Default is 1
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Component, Reflect)]
pub struct Cost(pub u32);

impl Default for Cost {
    fn default() -> Self {
        Self(1)
    }
}

/// Algorithm used to calculate the path between [`Start`] and [`Goal`]
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Resource, Reflect, Default)]
pub enum Algorithm {
    /// [Pathfinding A*](https://docs.rs/pathfinding/4.14.0/pathfinding/directed/astar/fn.astar.html)
    #[default]
    Astar,
}

/// Auxiliary struct that holds knowledge for path finding on each [`Cell`]
#[derive(Debug, Clone, PartialEq, Eq, Resource, Reflect, Default)]
pub struct MazePath {
    /// Path from
    pub path: Option<(Vec<CoordsComponent>, u32)>,
}

impl MazePath {
    /// Heuristic distance function to maze [`Goal`]
    pub const fn distance(coords: &CoordsComponent, other: &CoordsComponent) -> u32 {
        (coords.coord.0.abs_diff(other.coord.0) + coords.coord.1.abs_diff(other.coord.1)) as u32
    }

    /// Successor function of maze position
    pub fn successors(
        current: &CoordsComponent,
        cells: &HashMap<&CoordsComponent, (&Cell, Option<&Cost>)>,
    ) -> Vec<(CoordsComponent, u32)> {
        let Some((open_passages, cost)) = cells.get(&current) else {
            return Vec::default();
        };

        open_passages
            .iter()
            .map(|c| match c {
                Cell::EAST => (current.coord.0 + 1, current.coord.1),
                Cell::NORTH => (current.coord.0, current.coord.1 + 1),
                Cell::SOUTH => (current.coord.0, current.coord.1 - 1),
                Cell::WEST => (current.coord.0 - 1, current.coord.1),
                _ => current.coord,
            })
            .map(|coord| (coord.into(), cost.cloned().unwrap_or_default().0))
            .collect()
    }
}

#[cfg(not(tarpaulin_include))]
pub(crate) fn find_path(
    mut commands: Commands,
    start: Query<
        &CoordsComponent,
        (
            With<Cell>,
            With<Start>,
            Changed<CoordsComponent>,
            Changed<Cell>,
        ),
    >,
    goal: Query<
        &CoordsComponent,
        (
            With<Cell>,
            With<Goal>,
            Changed<CoordsComponent>,
            Changed<Cell>,
        ),
    >,
    cells: Query<(&CoordsComponent, &Cell, Option<&Cost>)>,
    _algorithm: Res<Algorithm>,
) {
    let Ok(start) = start.get_single().cloned() else {
        return;
    };
    let Ok(goal) = goal.get_single().cloned() else {
        return;
    };
    let cells: HashMap<&CoordsComponent, (&Cell, Option<&Cost>)> =
        cells.iter().map(|(k, v1, v2)| (k, (v1, v2))).collect();

    let path: Option<(Vec<CoordsComponent>, u32)> = astar(
        &start,
        |p| MazePath::successors(p, &cells),
        |p| MazePath::distance(p, &goal),
        |p| p == &goal,
    );

    commands.insert_resource(MazePath { path });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn coords_distance() {
        let start: CoordsComponent = (0, 0).into();
        let goal: CoordsComponent = (10, 10).into();

        let distance = MazePath::distance(&start, &goal);

        assert_eq!(distance, 20);
    }

    #[test]
    fn successors() {
        let start: CoordsComponent = (12, 12).into();
        let cell = Cell::from_bits(0b0101).unwrap();
        let key = CoordsComponent::new(12, 12);
        let cells = [(&key, (&cell, None::<&Cost>))].into_iter().collect();

        let successor = MazePath::successors(&start, &cells);

        assert_eq!(successor.len(), 2);
        assert_eq!(successor[0], ((12, 13).into(), 1));
        assert_eq!(successor[1], ((13, 12).into(), 1));
    }

    #[test]
    fn successors_with_cost() {
        let start: CoordsComponent = (10, 10).into();
        let cell = Cell::from_bits(0b1010).unwrap();
        let key = CoordsComponent::new(10, 10);
        let cells = [(&key, (&cell, Some(&Cost(2))))].into_iter().collect();

        let successor = MazePath::successors(&start, &cells);

        assert_eq!(successor.len(), 2);
        assert_eq!(successor[0], ((10, 9).into(), 2));
        assert_eq!(successor[1], ((9, 10).into(), 2));
    }

    #[test]
    fn empty_successors() {
        let goal: CoordsComponent = (0, 0).into();
        let cell = Cell::from_bits(0b0101).unwrap();
        let key = CoordsComponent::new(12, 12);
        let cells = [(&key, (&cell, None::<&Cost>))].into_iter().collect();

        let successor = MazePath::successors(&goal, &cells);

        assert_eq!(successor.len(), 0);
    }
}
