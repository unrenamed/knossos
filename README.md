[![Latest Version](https://img.shields.io/crates/v/knossos)](https://crates.io/crates/knossos)
[![License:Apache](https://img.shields.io/badge/License-Apache_2.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)
[![Build Status](https://github.com/unrenamed/knossos/actions/workflows/build.yml/badge.svg?branch=main)](https://github.com/unrenamed/knossos/actions/workflows/build.yml)
[![Coverage Status](https://coveralls.io/repos/github/unrenamed/knossos/badge.svg)](https://coveralls.io/github/unrenamed/knossos)

<p align="center">
  <img src="assets/daedalus.png?raw=true" width="400" height="400">
</p>

# Knossos

Knossos is a Rust library and CLI for generating mazes with some basic routines for rendering and saving mazes to files.

## Reference

In Greek mythology, King Minos dwelt in a palace at Knossos. He hired the Athenian architect, mathematician, and inventor Daedalus to design his palace and so cleverly was it constructed that no one who entered could find their way back out without a guide. In other versions of this same story it was not the palace itself which was designed in this way but the labyrinth within the palace which was built to house the half-man/half-bull the Minotaur. In order to keep Daedalus from telling the secrets of the palace, Minos locked him and his son Icarus in a high tower at Knossos and kept them prisoner. Daedalus fashioned wings made of wax and bird's feathers for himself and his son, however, and escaped their prison but Icarus, flying too close to the sun, melted his wings and fell to his death. 

Source: https://www.worldhistory.org/knossos

## Overview

Knossos currently supports only one type of mazes: **orthogonal**, which is a standard maze layout of rectangular passages.

The library supports the following generation algorithms:

* [Aldous-Broder](https://weblog.jamisbuck.org/2011/1/17/maze-generation-aldous-broder-algorithm)
* [Binary Tree](https://weblog.jamisbuck.org/2011/2/1/maze-generation-binary-tree-algorithm)
* [Eller's](https://weblog.jamisbuck.org/2010/12/29/maze-generation-eller-s-algorithm)
* [Growing Tree](https://weblog.jamisbuck.org/2011/1/27/maze-generation-growing-tree-algorithm)
* [Hunt-and-Kill](https://weblog.jamisbuck.org/2011/1/24/maze-generation-hunt-and-kill-algorithm)
* [Kruskal's](https://weblog.jamisbuck.org/2011/1/3/maze-generation-kruskal-s-algorithm)
* [Prim's](https://weblog.jamisbuck.org/2011/1/10/maze-generation-prim-s-algorithm)
* [Recursive Backtracking](https://weblog.jamisbuck.org/2010/12/27/maze-generation-recursive-backtracking)
* [Recursive Division](https://weblog.jamisbuck.org/2011/1/12/maze-generation-recursive-division-algorithm)
* [Sidewinder](https://weblog.jamisbuck.org/2011/2/3/maze-generation-sidewinder-algorithm)

Knossos supports the following output types:

* **ASCII** Using the ASCII output, you can simply print a maze to the console or write it to a file to see what it looks like

* **Game map** If you're interested in writing your own game with pseudo 3D graphics or just testing your implementation of the ray casting algorithm, you can convert a maze into a game map. Currently, this formatter supports one configuration option: it's a `span` value describing any passage — distance between two opposite walls.

* **Image** Using the Image output, you can render a maze to PNG or JPG (just use the corresponding filename extension). This type of output is highly customizable: it allows you to specify custom margin, wall and passage width, and even background and foreground colors

## Installation
Run the following Cargo command in your project directory:
```no_test
cargo add knossos
```

Or add the following line to your `Cargo.toml`:
```no_test
[dependencies]
knossos = "0.2.0"
```

## Usage

Knossos is designed to be super easy and convenient to use. Here are some usage examples of how to generate, display and save mazes:

### Generate with default parameters
```rust,no_run
use knossos::maze::*;

let maze = OrthogonalMazeBuilder::new().build();
```

### Generate with custom parameters
```rust,no_run
use knossos::maze::*;

let maze = OrthogonalMazeBuilder::new()
 .height(10)
 .width(10)
 .algorithm(Box::new(GrowingTree::new(Method::Random)))
 .build();
```

### Display
```rust,no_run
use knossos::maze::*;

let maze = OrthogonalMazeBuilder::new().build();
println!("{}", &maze);
```

### Save to file
```rust,no_run
use knossos::maze::*;

let maze = OrthogonalMazeBuilder::new().build();

// Save as ascii
maze.save("output/maze.txt", AsciiNarrow).unwrap();
// Save as a game map
maze.save("output/maze_game_map.txt", GameMap::new().span(3)).unwrap();
// Save as a PNG image
maze.save("output/maze.png", Image::new().wall(10).passage(30)).unwrap();
```

You can find more examples in [src/example.rs](src/example.rs)

## Benchmarks
```bash
cargo +nightly bench 
    Finished bench [optimized] target(s) in 0.51s
     Running unittests src/lib.rs (target/release/deps/knossos-43150be123983d04)

running 22 tests
test maze::builder::tests::build ... ignored
test maze::errors::save_error::tests::display ... ignored
test maze::errors::transit_error::tests::display ... ignored
test maze::formatters::ascii::tests::format_narrow ... ignored
test maze::formatters::ascii::tests::format_broad ... ignored
test maze::formatters::game_map::tests::format ... ignored
test maze::formatters::game_map::tests::new_call_default_params ... ignored
test maze::formatters::game_map::tests::passage_change ... ignored
test maze::formatters::game_map::tests::span_change ... ignored
test maze::formatters::game_map::tests::wall_change ... ignored
test maze::formatters::image::tests::format ... ignored
test maze::formatters::image::tests::new_call_default_params ... ignored
test maze::formatters::image::tests::params_change ... ignored
test maze::maze::tests::display_orthogonal_maze ... ignored
test maze::maze::tests::invalid_maze ... ignored
test maze::maze::tests::valid_maze ... ignored
test utils::arena::tests::connect_one_none_node ... ignored
test utils::arena::tests::connect_three_nodes ... ignored
test utils::arena::tests::connect_two_nodes ... ignored
test utils::arena::tests::connect_two_none_node ... ignored
test utils::arena::tests::unconnected_nodes ... ignored
test utils::color::tests::display_color ... ignored

test result: ok. 0 passed; 0 failed; 22 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running unittests src/main.rs (target/release/deps/knossos-c42e4506ba8fe599)

running 1 test
test tests::verify_cli ... ignored

test result: ok. 0 passed; 0 failed; 1 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running benches/algorithms.rs (target/release/deps/algorithms-99993a694f1f462a)

running 26 tests
test algorithms::aldous_broder::generate_100_x_100              ... bench:  25,114,116 ns/iter (+/- 8,976,086)
test algorithms::aldous_broder::generate_10_x_10                ... bench:      82,836 ns/iter (+/- 7,641)
test algorithms::binary_tree::generate_100_x_100                ... bench:     518,780 ns/iter (+/- 4,661)
test algorithms::binary_tree::generate_10_x_10                  ... bench:       5,278 ns/iter (+/- 44)
test algorithms::eller::generate_100_x_100                      ... bench:   2,924,531 ns/iter (+/- 1,151,159)
test algorithms::eller::generate_10_x_10                        ... bench:      26,874 ns/iter (+/- 922)
test algorithms::growing_tree_method_middle::generate_100_x_100 ... bench:   2,060,609 ns/iter (+/- 46,684)
test algorithms::growing_tree_method_middle::generate_10_x_10   ... bench:      20,162 ns/iter (+/- 154)
test algorithms::growing_tree_method_newest::generate_100_x_100 ... bench:   1,834,971 ns/iter (+/- 18,044)
test algorithms::growing_tree_method_newest::generate_10_x_10   ... bench:      19,742 ns/iter (+/- 5,935)
test algorithms::growing_tree_method_oldest::generate_100_x_100 ... bench:   2,059,533 ns/iter (+/- 118,326)
test algorithms::growing_tree_method_oldest::generate_10_x_10   ... bench:      20,145 ns/iter (+/- 1,032)
test algorithms::growing_tree_method_random::generate_100_x_100 ... bench:   2,553,812 ns/iter (+/- 390,277)
test algorithms::growing_tree_method_random::generate_10_x_10   ... bench:      24,224 ns/iter (+/- 6,544)
test algorithms::hunt_and_kill::generate_100_x_100              ... bench:   1,060,523 ns/iter (+/- 83,817)
test algorithms::hunt_and_kill::generate_10_x_10                ... bench:       9,249 ns/iter (+/- 301)
test algorithms::kruskal::generate_100_x_100                    ... bench:  38,615,035 ns/iter (+/- 23,011,185)
test algorithms::kruskal::generate_10_x_10                      ... bench:      19,223 ns/iter (+/- 909)
test algorithms::prim::generate_100_x_100                       ... bench:   5,987,027 ns/iter (+/- 2,530,387)
test algorithms::prim::generate_10_x_10                         ... bench:      21,962 ns/iter (+/- 8,735)
test algorithms::recursive_backtracking::generate_100_x_100     ... bench:   1,199,184 ns/iter (+/- 146,586)
test algorithms::recursive_backtracking::generate_10_x_10       ... bench:      11,472 ns/iter (+/- 2,661)
test algorithms::recursive_division::generate_100_x_100         ... bench:     349,725 ns/iter (+/- 19,278)
test algorithms::recursive_division::generate_10_x_10           ... bench:       3,511 ns/iter (+/- 57)
test algorithms::sidewinder::generate_100_x_100                 ... bench:     254,377 ns/iter (+/- 24,068)
test algorithms::sidewinder::generate_10_x_10                   ... bench:       2,526 ns/iter (+/- 17)

test result: ok. 0 passed; 0 failed; 0 ignored; 26 measured; 0 filtered out; finished in 71.73s

     Running benches/formatters.rs (target/release/deps/formatters-1b577342650eb048)

running 8 tests
test formatters::ascii_narrow::format_100_x_100  ... bench:    1,457,895 ns/iter (+/- 6,362)
test formatters::ascii_narrow::format_10_x_10    ... bench:       85,277 ns/iter (+/- 19,651)
test formatters::ascii_broad::format_100_x_100 ... bench:      1,344,412 ns/iter (+/- 5,683)
test formatters::ascii_broad::format_10_x_10   ... bench:         83,734 ns/iter (+/- 1,928)
test formatters::game_map::format_100_x_100       ... bench:   7,905,384 ns/iter (+/- 104,229)
test formatters::game_map::format_10_x_10         ... bench:     149,456 ns/iter (+/- 3,197)
test formatters::image::format_10_x_10            ... bench:  40,956,263 ns/iter (+/- 3,166,280)
test formatters::image::format_50_x_50            ... bench: 864,432,205 ns/iter (+/- 2,515,914)

test result: ok. 0 passed; 0 failed; 0 ignored; 8 measured; 0 filtered out; finished in 278.25s
```

## CLI
A command-line interface for generating mazes in the terminal uses the library's public API.

### Examples
```bash
knossos generate -W 5 -H 5 ascii --output-type=broad --output-path=maze.txt
+---+---+---+---+---+
|   |           |   |
+   +   +   +   +   +
|   |   |   |       |
+   +---+   +---+   +
|       |   |   |   |
+---+   +   +   +   +
|   |   |   |   |   |
+   +   +   +   +   +
|           |       |
+---+---+---+---+---+

```

```bash
knossos generate -W 5 -H 5 game-map --span 2 --output-path=maze.txt
################
#........#.....#
#........#.....#
#######..#..#..#
#........#..#..#
#........#..#..#
#..#######..####
#........#.....#
#........#.....#
#######..#..#..#
#.....#..#..#..#
#.....#..#..#..#
#..#..#..####..#
#..#...........#
#..#...........#
################
```

```bash
knossos generate -W 15 -H 15 image --output-path=maze.png
```
<img src="assets/maze.png?raw=true" width="300" height="300">


### Installation

Debian package:
1. [Download](https://github.com/unrenamed/knossos/releases) the latest binary
2. `dpkg -i <binary-name>` will install it

Or from crates.io:
```bash
cargo install knossos
```

Or from source:
```bash
$ git clone git@github.com:unrenamed/knossos.git
$ cd knossos
$ cargo build --release
$ ./target/release/knossos --version
knossos 0.1.2
```

### Usage
```bash
knossos --help
Rust library for generating and rendering mazes

Usage: knossos <COMMAND>

Commands:
  generate  Generates a maze
  help      Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version
```

**Using `generate` command:**
```bash
knossos generate -h
Generates a maze

Usage: knossos generate [OPTIONS] <COMMAND>

Commands:
  ascii     Save to a text file with an ASCII representation of a maze
  game-map  Save to a text file as an ASCII game map for pseudo 3D games that use ray casting for modeling and rendering the map
  image     Save to PNG or JPG file
  help      Print this message or the help of the given subcommand(s)

Options:
  -A, --algorithm <ALGORITHM>
          Maze generation algorithm [default: recursive-backtracking] [possible values: aldous-broder, binary-tree, eller, growing-tree, hunt-and-kill, kruskal, prim, recursive-backtracking, recursive-division, sidewinder]
  -H, --height <HEIGHT>
          Grid height in a number of cells [default: 10]
  -W, --width <WIDTH>
          Grid width in a number of cells [default: 10]
      --bias[=<BIAS>]
          Bias to use for the "Binary Tree" algorithm [default: north-east] [possible values: north-west, north-east, south-west, south-east]
      --growing-method[=<GROWING_METHOD>]
          Growing method to use for the "Growing Tree" algorithm [default: newest] [possible values: newest, oldest, random, middle, newest50-random50, newest75-random25, newest25-random75]
  -h, --help
          Print help (see more with '--help')
```

For more info, run 
```bash
knossos generate help
```
