//! Maze representations, builders, formatters, and supported algorithms for generating mazes
//!
//! Acts as a prelude module with all the imports that are necessary for generating and saving
//! mazes.

mod builder;
mod grid;
#[allow(clippy::module_inception)]
mod maze;
mod errors;
mod validate;

pub mod algorithms;
pub mod formatters;

pub use algorithms::*;
pub use builder::OrthogonalMazeBuilder;
pub use formatters::{AsciiNarrow, AsciiBroad, GameMap, Image};
pub use errors::MazeSaveError;
pub use maze::OrthogonalMaze;
