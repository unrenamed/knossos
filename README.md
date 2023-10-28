[![Latest Version](https://img.shields.io/crates/v/knossos)](https://crates.io/crates/knossos)
[![License:Apache](https://img.shields.io/badge/License-Apache_2.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)
[![Build Status](https://github.com/unrenamed/knossos/actions/workflows/build.yml/badge.svg?branch=main)](https://github.com/unrenamed/knossos/actions/workflows/build.yml)
[![Coverage Status](https://coveralls.io/repos/github/unrenamed/knossos/badge.svg)](https://coveralls.io/github/unrenamed/knossos)

<p align="center">
  <img src="assets/daedalus.png?raw=true" width="400" height="400">
</p>

# Knossos

Knossos is a Rust library and CLI for maze generation, complete with fundamental functions for rendering and saving mazes to files.

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

* **ASCII** With the ASCII output option, you can effortlessly display a maze on the console or save it to a file to visualize its appearance.

* **Game map** If you are looking to create your own game featuring pseudo 3D graphics or testing your ray casting algorithm implementation, you can transform a maze into a game map using this formatter. It offers various configuration options, including the `span` value for specifying the distance between opposing walls, the characters `wall` and `passage` for map construction, and the ability to randomly place start `S` and goal `G` points along the borders.

* **Image** Utilizing the Image output feature, you have the capability to render a maze into PNG or JPG formats (simply utilize the appropriate filename extension). This output type offers extensive customization options, enabling you to define custom margins, wall and passage widths, as well as background and foreground colors.

## Installation
Run the following Cargo command in your project directory:
```no_test
cargo add knossos
```

Or add the following line to your `Cargo.toml`:
```no_test
[dependencies]
knossos = "0.3.0"
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

You can find more examples in the [examples](examples) directory. To run the example:
```bash
cargo run --example mazes
```

## Benchmarks
```bash
cargo +nightly bench 
    Finished bench [optimized] target(s) in 0.51s
     Running unittests src/lib.rs (target/release/deps/knossos-43150be123983d04)

running 29 tests
test maze::algorithms::prim::tests::default_call ... ignored
test maze::builder::tests::build ... ignored
test maze::errors::save_error::tests::display ... ignored
test maze::errors::transit_error::tests::display ... ignored
test maze::formatters::ascii::tests::format_broad ... ignored
test maze::formatters::ascii::tests::format_narrow ... ignored
test maze::formatters::game_map::tests::default_call ... ignored
test maze::formatters::game_map::tests::format_with_no_start_and_goal ... ignored
test maze::formatters::game_map::tests::format_with_start_and_goal ... ignored
test maze::formatters::game_map::tests::goal_change ... ignored
test maze::formatters::game_map::tests::new_call ... ignored
test maze::formatters::game_map::tests::passage_change ... ignored
test maze::formatters::game_map::tests::possible_start_and_goal_positions ... ignored
test maze::formatters::game_map::tests::possible_start_and_goal_positions_when_map_is_empty ... ignored
test maze::formatters::game_map::tests::span_change ... ignored
test maze::formatters::game_map::tests::start_change ... ignored
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

test result: ok. 0 passed; 0 failed; 29 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running unittests src/main.rs (target/release/deps/knossos-d8bfa62e62b46d44)

running 1 test
test tests::verify_cli ... ignored

test result: ok. 0 passed; 0 failed; 1 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running benches/algorithms.rs (target/release/deps/algorithms-e8ead48e0698db1d)

running 26 tests
test algorithms::aldous_broder::generate_100_x_100              ... bench:  27,608,666 ns/iter (+/- 9,378,586)
test algorithms::aldous_broder::generate_10_x_10                ... bench:      90,247 ns/iter (+/- 4,901)
test algorithms::binary_tree::generate_100_x_100                ... bench:     518,312 ns/iter (+/- 28,780)
test algorithms::binary_tree::generate_10_x_10                  ... bench:       5,126 ns/iter (+/- 318)
test algorithms::eller::generate_100_x_100                      ... bench:   2,195,923 ns/iter (+/- 172,412)
test algorithms::eller::generate_10_x_10                        ... bench:      22,002 ns/iter (+/- 508)
test algorithms::growing_tree_method_middle::generate_100_x_100 ... bench:   1,942,837 ns/iter (+/- 69,464)
test algorithms::growing_tree_method_middle::generate_10_x_10   ... bench:      18,608 ns/iter (+/- 385)
test algorithms::growing_tree_method_newest::generate_100_x_100 ... bench:   1,721,368 ns/iter (+/- 51,877)
test algorithms::growing_tree_method_newest::generate_10_x_10   ... bench:      17,912 ns/iter (+/- 931)
test algorithms::growing_tree_method_oldest::generate_100_x_100 ... bench:   1,959,410 ns/iter (+/- 106,218)
test algorithms::growing_tree_method_oldest::generate_10_x_10   ... bench:      18,575 ns/iter (+/- 614)
test algorithms::growing_tree_method_random::generate_100_x_100 ... bench:   2,621,709 ns/iter (+/- 146,396)
test algorithms::growing_tree_method_random::generate_10_x_10   ... bench:      22,979 ns/iter (+/- 507)
test algorithms::hunt_and_kill::generate_100_x_100              ... bench:     913,686 ns/iter (+/- 29,865)
test algorithms::hunt_and_kill::generate_10_x_10                ... bench:       8,736 ns/iter (+/- 140)
test algorithms::kruskal::generate_100_x_100                    ... bench:  47,468,245 ns/iter (+/- 5,990,785)
test algorithms::kruskal::generate_10_x_10                      ... bench:      12,918 ns/iter (+/- 765)
test algorithms::prim::generate_100_x_100                       ... bench:   3,038,950 ns/iter (+/- 245,608)
test algorithms::prim::generate_10_x_10                         ... bench:      13,750 ns/iter (+/- 667)
test algorithms::recursive_backtracking::generate_100_x_100     ... bench:   1,017,889 ns/iter (+/- 52,493)
test algorithms::recursive_backtracking::generate_10_x_10       ... bench:      10,304 ns/iter (+/- 488)
test algorithms::recursive_division::generate_100_x_100         ... bench:     353,212 ns/iter (+/- 9,616)
test algorithms::recursive_division::generate_10_x_10           ... bench:       3,551 ns/iter (+/- 83)
test algorithms::sidewinder::generate_100_x_100                 ... bench:     267,238 ns/iter (+/- 10,063)
test algorithms::sidewinder::generate_10_x_10                   ... bench:       2,756 ns/iter (+/- 112)

test result: ok. 0 passed; 0 failed; 0 ignored; 26 measured; 0 filtered out; finished in 82.88s

     Running benches/formatters.rs (target/release/deps/formatters-a575860cb997bf40)

running 8 tests
test formatters::ascii_broad::format_100_x_100  ... bench:   1,275,793 ns/iter (+/- 56,448)
test formatters::ascii_broad::format_10_x_10    ... bench:     173,265 ns/iter (+/- 18,587)
test formatters::ascii_narrow::format_100_x_100 ... bench:   1,338,637 ns/iter (+/- 103,429)
test formatters::ascii_narrow::format_10_x_10   ... bench:     173,031 ns/iter (+/- 27,930)
test formatters::game_map::format_100_x_100     ... bench:   3,776,375 ns/iter (+/- 243,290)
test formatters::game_map::format_10_x_10       ... bench:     199,187 ns/iter (+/- 20,521)
test formatters::image::format_10_x_10          ... bench:   8,341,370 ns/iter (+/- 349,585)
test formatters::image::format_50_x_50          ... bench: 191,061,562 ns/iter (+/- 2,174,934)

test result: ok. 0 passed; 0 failed; 0 ignored; 8 measured; 0 filtered out; finished in 77.40s
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
knossos generate -W 5 -H 5 game-map --span 2 --with-start-goal --output-path=maze.txt
#######S########
#..#...........#
#..#...........#
#..#..#######..#
#..#........#..#
#..#........#..#
G..##########..#
#..#........#..#
#..#........#..#
#..#..####..#..#
#.....#.....#..#
#.....#.....#..#
#######..####..#
#..............#
#..............#
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
