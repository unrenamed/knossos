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
knossos = "0.1.2"
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
maze.save("output/maze.txt", Ascii).unwrap();
// Save as a game map
maze.save("output/maze_game_map.txt", GameMap::new().span(3)).unwrap();
// Save as a PNG image
maze.save("output/maze.png", Image::new().wall(10).passage(30)).unwrap();
```

You can find more examples in [src/example.rs](src/example.rs)

## Benchmarks
```bash
cargo +nightly bench
   Compiling knossos v0.1.2 (/home/nazar-home/Documents/dev/rust/knossos)
    Finished bench [optimized] target(s) in 2.59s
     Running unittests src/lib.rs (target/release/deps/knossos-b73d675c8a8d5427)

running 22 tests
test maze::builder::tests::build ... ignored
test maze::errors::save_error::tests::display ... ignored
test maze::errors::transit_error::tests::display ... ignored
test maze::formatters::ascii::tests::format_default ... ignored
test maze::formatters::ascii::tests::format_ehanced ... ignored
test maze::formatters::game_map::tests::format ... ignored
test maze::formatters::game_map::tests::new_call_default_params ... ignored
test maze::formatters::game_map::tests::span_change ... ignored
test maze::formatters::image::tests::format ... ignored
test maze::formatters::image::tests::new_call_default_params ... ignored
test maze::formatters::image::tests::params_change ... ignored
test maze::grid::walls::tests::add_wall ... ignored
test maze::grid::walls::tests::build ... ignored
test maze::grid::walls::tests::build_empty ... ignored
test maze::grid::walls::tests::is_wall_carved ... ignored
test maze::grid::walls::tests::remove_wall ... ignored
test utils::arena::tests::connect_one_none_node ... ignored
test utils::arena::tests::connect_three_nodes ... ignored
test utils::arena::tests::connect_two_nodes ... ignored
test utils::arena::tests::connect_two_none_node ... ignored
test utils::arena::tests::unconnected_nodes ... ignored
test utils::num::tests::it_adds_numbers ... ignored

test result: ok. 0 passed; 0 failed; 22 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running unittests src/main.rs (target/release/deps/knossos-262c03a544ace7be)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running benches/algorithms.rs (target/release/deps/algorithms-bcd11aa94cd6bfd7)

running 26 tests
test algorithms::aldous_broder::generate_100_x_100                       ... bench:  45,266,902 ns/iter (+/- 14,973,744)
test algorithms::aldous_broder::generate_10_x_10                         ... bench:     150,370 ns/iter (+/- 12,344)
test algorithms::binary_tree::generate_100_x_100                         ... bench:   1,832,534 ns/iter (+/- 149,965)
test algorithms::binary_tree::generate_10_x_10                           ... bench:      17,417 ns/iter (+/- 377)
test algorithms::eller::generate_100_x_100                               ... bench:   4,331,760 ns/iter (+/- 86,095)
test algorithms::eller::generate_10_x_10                                 ... bench:      39,670 ns/iter (+/- 1,653)
test algorithms::growing_tree_hunt_and_kill::generate_100_x_100          ... bench:   6,223,144 ns/iter (+/- 3,055,071)
test algorithms::growing_tree_hunt_and_kill::generate_10_x_10            ... bench:      33,034 ns/iter (+/- 2,883)
test algorithms::growing_tree_kruskal::generate_100_x_100                ... bench:  37,008,342 ns/iter (+/- 6,333,645)
test algorithms::growing_tree_kruskal::generate_10_x_10                  ... bench:      36,099 ns/iter (+/- 2,789)
test algorithms::growing_tree_method_middle::generate_100_x_100          ... bench:   6,053,555 ns/iter (+/- 565,047)
test algorithms::growing_tree_method_middle::generate_10_x_10            ... bench:      55,696 ns/iter (+/- 785)
test algorithms::growing_tree_method_newest::generate_100_x_100          ... bench:   5,611,988 ns/iter (+/- 669,954)
test algorithms::growing_tree_method_newest::generate_10_x_10            ... bench:      54,763 ns/iter (+/- 1,122)
test algorithms::growing_tree_method_oldest::generate_100_x_100          ... bench:   5,982,678 ns/iter (+/- 219,966)
test algorithms::growing_tree_method_oldest::generate_10_x_10            ... bench:      55,528 ns/iter (+/- 1,363)
test algorithms::growing_tree_method_random::generate_100_x_100          ... bench:   6,405,201 ns/iter (+/- 157,112)
test algorithms::growing_tree_method_random::generate_10_x_10            ... bench:      58,219 ns/iter (+/- 1,033)
test algorithms::growing_tree_prim::generate_100_x_100                   ... bench:   7,043,416 ns/iter (+/- 695,387)
test algorithms::growing_tree_prim::generate_10_x_10                     ... bench:      32,192 ns/iter (+/- 498)
test algorithms::growing_tree_recursive_backtracking::generate_100_x_100 ... bench:   4,161,864 ns/iter (+/- 287,568)
test algorithms::growing_tree_recursive_backtracking::generate_10_x_10   ... bench:      40,214 ns/iter (+/- 1,304)
test algorithms::growing_tree_recursive_division::generate_100_x_100     ... bench:   2,159,627 ns/iter (+/- 18,530)
test algorithms::growing_tree_recursive_division::generate_10_x_10       ... bench:      18,816 ns/iter (+/- 262)
test algorithms::growing_tree_sidewinder::generate_100_x_100             ... bench:   1,595,038 ns/iter (+/- 26,018)
test algorithms::growing_tree_sidewinder::generate_10_x_10               ... bench:      14,648 ns/iter (+/- 229)

test result: ok. 0 passed; 0 failed; 0 ignored; 26 measured; 0 filtered out; finished in 80.51s

     Running benches/formatters.rs (target/release/deps/formatters-b5dfc932fa0358ec)

running 8 tests
test formatters::ascii_default::format_100_x_100  ... bench:   4,948,109 ns/iter (+/- 94,340)
test formatters::ascii_default::format_10_x_10    ... bench:      52,332 ns/iter (+/- 1,220)
test formatters::ascii_enhanced::format_100_x_100 ... bench:   4,619,765 ns/iter (+/- 28,018)
test formatters::ascii_enhanced::format_10_x_10   ... bench:      50,595 ns/iter (+/- 968)
test formatters::game_map::format_100_x_100       ... bench:  13,416,716 ns/iter (+/- 711,886)
test formatters::game_map::format_10_x_10         ... bench:     140,390 ns/iter (+/- 10,798)
test formatters::image::format_10_x_10            ... bench:  38,881,197 ns/iter (+/- 1,334,992)
test formatters::image::format_50_x_50            ... bench: 994,273,476 ns/iter (+/- 88,990,280)

test result: ok. 0 passed; 0 failed; 0 ignored; 8 measured; 0 filtered out; finished in 321.34s
```

## CLI
A command-line interface for generating mazes in the terminal uses the library's public API.

### Examples
```bash
knossos generate -W 5 -H 5 ascii --output-type=enhanced --output-path=maze.txt
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

Ubuntu:
1. [Download](https://github.com/unrenamed/terminal-fps/releases) the latest binary
2. `dpkg -i <binary-name>` will install it

From crates.io:
```bash
cargo install knossos
```

From source:
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
