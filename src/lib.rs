#![deny(missing_docs)]
//! # Overview
//!
//! A simple library for generating mazes with some basic routines for rendering and saving mazes to
//! files.
//!
//! # Installation
//! Run the following Cargo command in your project directory:
//! ```no_test
//! cargo add knossos
//! ```
//!
//! Or add the following line to your `Cargo.toml`:
//! ```no_test
//! [dependencies]
//! knossos = "0.4.0"
//! ```
//!
//! # Usage
//!
//! This crate is designed to be super easy to use. Here are some usage examples of how to generate,
//! display and save mazes:
//!
//! ## Generate with default parameters
//! ```rust,no_run
//! use bevy_knossos::maze::*;
//!
//! let maze = OrthogonalMazeBuilder::new().build();
//! ```
//!
//! ## Generate with custom parameters
//! ```rust,no_run
//! use bevy_knossos::maze::*;
//!
//! let maze = OrthogonalMazeBuilder::new()
//!  .height(10)
//!  .width(10)
//!  .algorithm(Box::new(GrowingTree::new(Method::Random)))
//!  .build();
//! ```
//!
//! Read more about [maze builder API](maze::OrthogonalMazeBuilder)
//!
//! ## Display
//! ```rust,no_run
//! use bevy_knossos::maze::*;
//!
//! let maze = OrthogonalMazeBuilder::new().build().unwrap();
//! println!("{}", &maze);
//! ```
//!
//! ## Save to file
//! ```rust,no_run
//! use bevy_knossos::maze::*;
//!
//! let maze = OrthogonalMazeBuilder::new().build().unwrap();
//!
//! // Save as ascii
//! maze.save("output/maze.txt", AsciiNarrow).unwrap();
//! // Save as a game map
//! maze.save("output/maze_game_map.txt", GameMap::new().span(3)).unwrap();
//! // Save as a PNG image
//! maze.save("output/maze.png", Image::new().wall(10).passage(30)).unwrap();
//! ```
//!
//! Read more about [maze formatters](maze::formatters)
//!
//! # Algorithms
//!
//! You can find 10 different algorithms supported by this crate. Each of them has its own pros and
//! cons: some of them are impressively efficient, some of them are slower but generate splendid
//! mazes that look hard to puzzle out, and others are extremely flexible and customizable. Do give
//! each of them a shot and find the best one that suits you:
//!
//! - [`AldousBroder`](maze::AldousBroder)
//! - [`BinaryTree`](maze::BinaryTree)
//! - [`Eller`](maze::Eller)
//! - [`GrowingTree`](maze::GrowingTree)
//! - [`HuntAndKill`](maze::HuntAndKill)
//! - [`Kruskal`](maze::Kruskal)
//! - [`Prim`](maze::Prim)
//! - [`RecursiveBacktracking`](maze::RecursiveBacktracking)
//! - [`RecursiveDivision`](maze::RecursiveDivision)
//! - [`Sidewinder`](maze::Sidewinder)

mod utils;

pub mod maze;
use bevy::app::Plugin;
use maze::Cell;
pub use utils::color::Color;
pub use utils::types::{Coords, CellSize, CoordsComponent, Goal, Start};

#[cfg(feature = "pathfinding")]
/// Module containing all necessary tooling to pathfind between [`Start`] and [`Goal`]
pub mod pathfind;

/// Plugin registering Knossos `Reflect` Components and Resources
pub struct KnossosPlugin;

#[cfg(not(tarpaulin_include))]
impl Plugin for KnossosPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.register_type::<CoordsComponent>()
            .register_type::<Cell>()
            .register_type::<Start>()
            .register_type::<Goal>()
            .register_type::<CellSize>();

        #[cfg(feature = "pathfinding")]
        {
            use bevy::app::Update;

            app.register_type::<pathfind::MazePath>()
                .register_type::<pathfind::MazeEndsPaths>()
                .register_type::<pathfind::MazeEnd>()
                .register_type::<pathfind::Algorithm>()
                .init_resource::<pathfind::Algorithm>()
                .init_resource::<pathfind::MazePath>()
                .init_resource::<pathfind::MazeEndsPaths>()
                .add_systems(Update, pathfind::find_path);
        }
    }
}
