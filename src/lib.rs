//! # Overview
//!
//! A simple library for generating mazes with some basic routines for rendering and saving mazes to
//! files.
//!
//! # Installation
//! Add this to your `Cargo.toml`
//!
//! ```no_test
//! [dependencies]
//! knossos = "0.1.2"
//! ```
//!
//! # Usage
//!
//! This crate is designed to be super easy to use. Here are some usage examples of how to generate,
//! display and save mazes:
//!
//! ## Generate with default parameters
//! ```rust,no_run
//! use knossos::maze::*;
//!
//! let maze = OrthogonalMazeBuilder::new().build();
//! ```
//!
//! ## Generate with custom parameters
//! ```rust,no_run
//! use knossos::maze::*;
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
//! use knossos::maze::*;
//!
//! let maze = OrthogonalMazeBuilder::new().build();
//! println!("{}", &maze);
//! ```
//!
//! ## Save to file
//! ```rust,no_run
//! use knossos::maze::*;
//!
//! let maze = OrthogonalMazeBuilder::new().build();
//!
//! // Save as ascii
//! maze.save("output/maze.txt", Ascii::<formatters::Default>::new()).unwrap();
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

pub use utils::color::Color;
