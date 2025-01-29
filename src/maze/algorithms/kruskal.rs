use rand::prelude::SliceRandom;

use super::Algorithm;
use crate::maze::grid::cell::Cell;
use crate::maze::grid::Grid;
use crate::utils::arena::{ArenaTree, NodeId};

type Edge = (usize, usize, Cell);
type Edges = Vec<Edge>;

/// The Kruskal's algorithm for generating mazes
///
/// Kruskal’s algorithm is a method for producing a minimal spanning tree from a weighted graph.
/// The randomized version of it can be used for generating a rather convincing maze very
/// effectively.
pub struct Kruskal;

/// An implementation of the Kruskal's algorithm for generating mazes
///
/// The randomized variation of the Kruskal's algorithm looks as follows:
///
/// 1. Throw all the edges in the graph into a set.
///
/// 2. Pull out the edge at random. If the edge connects two disjoint trees, join the trees.
///    Otherwise, throw that edge away.
///
/// 3. Repeat until there are no more edges left in the set.
impl Algorithm for Kruskal {
    fn generate(&mut self, grid: &mut Grid) {
        let mut arena = populate_arena(grid);
        let mut edges: Edges = populate_edges(grid);
        edges.shuffle(&mut rand::rng());

        while !edges.is_empty() {
            let edge: Option<Edge> = edges.pop();
            if edge.is_none() {
                break;
            }

            let (x, y, direction) = edge.unwrap();
            let (nx, ny) = grid.get_next_cell_coords((x, y), direction).unwrap();

            let node1 = NodeId(y * grid.width() + x);
            let node2 = NodeId(ny * grid.width() + nx);
            if !arena.connected(node1, node2) {
                arena.connect(node1, node2);
                grid.carve_passage((x, y), direction).unwrap();
            }
        }
    }
}

fn populate_arena(grid: &Grid) -> ArenaTree {
    let mut arena = ArenaTree::new();
    for _ in 0..grid.width() * grid.height() {
        arena.new_node();
    }
    arena
}

fn populate_edges(grid: &Grid) -> Edges {
    let mut edges: Vec<Edge> = vec![];

    for y in 0..grid.height() {
        for x in 0..grid.width() {
            if y > 0 {
                edges.push((x, y, Cell::NORTH))
            }
            if x > 0 {
                edges.push((x, y, Cell::WEST))
            }
        }
    }
    edges
}
