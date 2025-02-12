use bevy::prelude::*;
use pathfinding::prelude::astar;
use crate::{
    maze::{Cell, OrthogonalMaze},
    pathfind::{MazePath, Cost},
    Coords, CoordsComponent, Start,
};

use std::collections::HashMap;

/// Auxiliary struct that holds knowledge for path finding on each [`Cell`]
#[derive(Debug, Clone, PartialEq, Resource, Default, Reflect)]
pub struct MazeEndsPaths {
    /// Map containing all ends and their paths from the [`Start`] component [`Coords`]
    pub paths: HashMap<(Coords, Coords), (Vec<CoordsComponent>, u32)>,
}

/// Component that signals that the cell is a Maze End.
#[derive(Debug, Clone, PartialEq, Eq, Component, Default, Reflect)]
pub struct MazeEnd;

/// Creates resource [`MazeEndsPaths`] that defines paths for all ends in the maze.
/// This function should be called on demand and is not scheduled to run.
///
/// # Warning
/// This operation is quite slow for large mazes, as it needs to pathfind over all ends.
/// issue
#[cfg(not(tarpaulin_include))]
pub fn find_maze_ends_paths(
    mut commands: Commands,
    start: Query<&CoordsComponent, (With<Cell>, With<Start>)>,
    cells: Query<(Entity, &CoordsComponent, &Cell, Option<&Cost>)>,
    maze: Res<OrthogonalMaze>,
) {
    let Ok(start) = start.get_single().cloned() else {
        return;
    };

    let ends = maze.ends();

    for (entity, coords, ..) in &cells {
        if ends.iter().any(|x| x.0 == coords.xy()) {
            commands.entity(entity).insert(MazeEnd);
        }
    }

    let cells: HashMap<&CoordsComponent, (&Cell, Option<&Cost>)> = cells
        .iter()
        .map(|(_entity, k, v1, v2)| (k, (v1, v2)))
        .collect();

    let paths = ends
        .into_iter()
        .filter_map(|(goal, _cell)| {
            let goal_comp: CoordsComponent = goal.into();

            Some((
                (start.clone().into(), goal),
                astar(
                    &start,
                    |p| MazePath::successors(p, &cells),
                    |p| MazePath::distance(p, &goal_comp),
                    |p| p == &goal_comp,
                )?,
            ))
        })
        .collect();

    commands.insert_resource(MazeEndsPaths { paths });
}
