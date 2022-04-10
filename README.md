
[![License](https://img.shields.io/badge/License-Apache_2.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)
[![Rust](https://github.com/unrenamed/daedalus/actions/workflows/rust.yml/badge.svg?branch=main)](https://github.com/unrenamed/daedalus/actions/workflows/rust.yml)

<p align="center">
  <img src="https://docs.google.com/drawings/d/e/2PACX-1vQzI7Wxv9t3KbstS4eOsVcWArjIeTTclzIyJn-DGMcFIPBolrPRf_Ecfyamhy2JW8gu2G46ordlEyN4/pub?w=400&h=400"/>
</p>

# Daedalus

Daedalus is a Rust library for generating mazes with some basic routines for rendering and saving mazes to files.

### Reference

Daedalus was an inventor, craftsman, architect and artist in Greek mythology, who had two sons, Icarus and Iapyx.

He is best known as the creator of the Labyrinth, a huge maze located under the court of King Minos of Crete, where the Minotaur, a half-man half-bull creature dwelt. According to the myth, the king of Athens was forced to pay tribute to King Minos by sending seven young men and seven young women each year to Crete, in order to be sacrificed to the Minotaur.

Source: https://www.greekmythology.com/Myths/Mortals/Daedalus/daedalus.html

### Overview

Daedalus currently supports only one type of mazes: **orthogonal**, which is a standard maze layout of rectangular passages.

The library supports the following generation algorithms:

* **AldouBroder**
* **BinaryTree**
* **Eller**
* **GrowingTree**
* **HuntAndKill**
* **Kruskal**
* **Prim**
* **RecursiveBacktracking**
* **RecursiveDivision**
* **Sidewinder**

Daedalus supports the following output types:

* **ASCII** Using the ASCII output, you can simply print a maze to the console or write it to a file to see what it looks like

* **GameMap** If you're interested in writing your own game with pseudo 3D graphics or just testing your implementation of the ray casting algorithm, you can convert a maze into a game map. Currently, this formatter supports several colors for maze walls that are randomly set: R (red), G (green), B (blue), Y (yellow), and O (orange)

* **Image** Using the Image output, you can render a maze to PNG or JPG (just use the corresponding filename extension). This type of output is highly customizable: it allows you to specify custom margin, wall and passage width, and even background and foreground colors

### Usage

Daedalus is designed to be super easy and convenient to use. Here are some usage examples of how to generate, display and save mazes:

#### Generate with default parameters
```rust,no_run
use daedalus::maze::*;

let maze = OrthogonalMazeBuilder::new().build();
```

#### Generate with custom parameters
```rust,no_run
use daedalus::maze::*;

let maze = OrthogonalMazeBuilder::new()
 .height(10)
 .width(10)
 .algorithm(Box::new(GrowingTree::new(Method::Random)))
 .build();
```

#### Display
```rust,no_run
use daedalus::maze::*;

let maze = OrthogonalMazeBuilder::new().build();
println!("{}", &maze);
```

#### Save to file
```rust,no_run
use daedalus::maze::*;

let maze = OrthogonalMazeBuilder::new().build();

// Save as ascii
maze.save("output/maze.txt", Ascii).unwrap();
// Save as a game map
maze.save("output/maze_game_map.txt", GameMap::new().span(3)).unwrap();
// Save as a PNG image
maze.save("output/maze.png", Image::new().wall(10).passage(30)).unwrap();
```

You can find more examples in [src/main.rs](src/main.rs)
