//! # Overview
//!
//! `knossos` is a simple and flexible library for generating mazes using various algorithms.
//! It provides built-in utilities for rendering mazes as ASCII, images, or game maps, as well
//! as saving them to files in multiple formats.
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
//! knossos = "1.2.0"
//! ```
//!
//! # Usage
//!
//! This crate is designed to be super easy to use. Here are some usage examples of how to generate,
//! display and save mazes:
//!
//! ## Generate with Default Parameters
//! ```rust,no_run
//! use knossos::maze::*;
//!
//! let maze = OrthogonalMazeBuilder::new().build();
//! ```
//!
//! ## Generate with Custom Parameters
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
//! ## Display Mazes
//! ```rust,no_run
//! use knossos::maze::*;
//!
//! let maze = OrthogonalMazeBuilder::new().build();
//! println!("{}", &maze);
//! ```
//!
//! ## Save to File
//! ```rust,no_run
//! use knossos::maze::*;
//!
//! let maze = OrthogonalMazeBuilder::new().build();
//!
//! // Save as ASCII text
//! maze.save("output/maze.txt", AsciiNarrow).unwrap();
//! // Save as a game map (with adjustable span size)
//! maze.save("output/maze_game_map.txt", GameMap::new().span(3)).unwrap();
//! // Save as a PNG image (adjusting wall and passage sizes)
//! maze.save("output/maze.png", Image::new().wall(10).passage(30)).unwrap();
//! ```
//!
//! ## Format for Further Processing or Logging
//! ```rust,no_run
//! use knossos::maze::*;
//!
//! let maze = OrthogonalMazeBuilder::new().build();
//!
//! // Convert to ASCII text
//! let ascii = maze.format(AsciiNarrow).into_inner();
//! // Convert to a game map
//! let game_map = maze.format(GameMap::new()).into_inner();
//! // Convert to an RGB image buffer
//! let rgb_image = maze.format(Image::new().wall(10).passage(30)).into_inner();
//! ```
//!
//! Read more about [maze formatters](maze::formatters)
//!
//! ## Seeding for Deterministic Mazes
//!
//! By default, each generated maze is randomized, producing a different layout every time. However,
//! you can use a seed value to ensure that the same maze is generated consistently across runs.
//! This is useful for debugging, testing, or sharing the exact same maze with others.
//!
//! ```rust,no_run
//! use knossos::maze::*;
//!
//! // Generate a maze with a fixed seed
//! let maze = OrthogonalMazeBuilder::new().seed(Some(40)).build();
//! ```
//!
//! Passing `None` as the seed (or omitting the `.seed()` method) will result in a random maze each time.
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
