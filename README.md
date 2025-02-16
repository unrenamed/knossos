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

- [Aldous-Broder](https://weblog.jamisbuck.org/2011/1/17/maze-generation-aldous-broder-algorithm)
- [Binary Tree](https://weblog.jamisbuck.org/2011/2/1/maze-generation-binary-tree-algorithm)
- [Eller's](https://weblog.jamisbuck.org/2010/12/29/maze-generation-eller-s-algorithm)
- [Growing Tree](https://weblog.jamisbuck.org/2011/1/27/maze-generation-growing-tree-algorithm)
- [Hunt-and-Kill](https://weblog.jamisbuck.org/2011/1/24/maze-generation-hunt-and-kill-algorithm)
- [Kruskal's](https://weblog.jamisbuck.org/2011/1/3/maze-generation-kruskal-s-algorithm)
- [Prim's](https://weblog.jamisbuck.org/2011/1/10/maze-generation-prim-s-algorithm)
- [Recursive Backtracking](https://weblog.jamisbuck.org/2010/12/27/maze-generation-recursive-backtracking)
- [Recursive Division](https://weblog.jamisbuck.org/2011/1/12/maze-generation-recursive-division-algorithm)
- [Sidewinder](https://weblog.jamisbuck.org/2011/2/3/maze-generation-sidewinder-algorithm)

Knossos supports the following output types:

- **ASCII** With the ASCII output option, you can effortlessly display a maze on the console or save it to a file to visualize its appearance.

- **Game map** If you are looking to create your own game featuring pseudo 3D graphics or testing your ray casting algorithm implementation, you can transform a maze into a game map using this formatter. It offers various configuration options, including the `span` value for specifying the distance between opposing walls, the characters `wall` and `passage` for map construction, and the ability to randomly place start `S` and goal `G` points along the borders.

- **Image** Utilizing the Image output feature, you have the capability to render a maze into PNG or JPG formats (simply utilize the appropriate filename extension). This output type offers extensive customization options, enabling you to define custom margins, wall and passage widths, as well as background and foreground colors.

## Installation

Run the following Cargo command in your project directory:

```no_test
cargo add knossos
```

Or add the following line to your `Cargo.toml`:

```no_test
[dependencies]
knossos = "0.4.0"
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

### Get a formatted maze

```rust,no_run
use knossos::maze::*;

let maze = OrthogonalMazeBuilder::new().build();

// Format as ascii
let ascii = maze.format(AsciiNarrow).into_inner();
// Format as a game map
let game_map = maze.format(GameMap::new()).into_inner();
// Format as a PNG image
let rgb_image = maze.format(Image::new().wall(10).passage(30)).
```

You can find more examples in the [examples](examples) directory. To run the example:

```bash
cargo run --example mazes
```

## Benchmarks

```bash
cargo bench
    Finished `bench` profile [optimized] target(s) in 0.06s
     Running unittests src/lib.rs (target/release/deps/knossos-7534cc8725e7cb1e)

running 31 tests
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
test maze::formatters::tests::into_inner_returns_inner_image ... ignored
test maze::formatters::tests::into_inner_returns_inner_string ... ignored
test maze::maze::tests::display_orthogonal_maze ... ignored
test maze::maze::tests::invalid_maze ... ignored
test maze::maze::tests::valid_maze ... ignored
test utils::arena::tests::connect_one_none_node ... ignored
test utils::arena::tests::connect_three_nodes ... ignored
test utils::arena::tests::connect_two_nodes ... ignored
test utils::arena::tests::connect_two_none_node ... ignored
test utils::arena::tests::unconnected_nodes ... ignored
test utils::color::tests::display_color ... ignored

test result: ok. 0 passed; 0 failed; 31 ignored; 0 measured; 0 filtered out; finished in 0.00s


running 1 test
test tests::verify_cli ... ignored

test result: ok. 0 passed; 0 failed; 1 ignored; 0 measured; 0 filtered out; finished in 0.00s

aldous_broder/generate_10_x_10
                        time:   [45.147 µs 45.694 µs 46.427 µs]
Found 10 outliers among 100 measurements (10.00%)
  1 (1.00%) low mild
  1 (1.00%) high mild
  8 (8.00%) high severe

aldous_broder/generate_100_x_100
                        time:   [13.535 ms 13.789 ms 14.049 ms]
Found 1 outliers among 100 measurements (1.00%)
  1 (1.00%) high mild

binary_tree/generate_10_x_10
                        time:   [4.0307 µs 4.0387 µs 4.0526 µs]
Found 5 outliers among 100 measurements (5.00%)
  1 (1.00%) high mild
  4 (4.00%) high severe

binary_tree/generate_100_x_100
                        time:   [398.05 µs 398.12 µs 398.21 µs]
Found 11 outliers among 100 measurements (11.00%)
  5 (5.00%) high mild
  6 (6.00%) high severe

eller/generate_10_x_10  time:   [23.606 µs 23.707 µs 23.807 µs]

eller/generate_100_x_100
                        time:   [2.2672 ms 2.2689 ms 2.2706 ms]
Found 3 outliers among 100 measurements (3.00%)
  1 (1.00%) high mild
  2 (2.00%) high severe

growing_tree_method_random/generate_10_x_10
                        time:   [12.795 µs 12.805 µs 12.819 µs]
Found 7 outliers among 100 measurements (7.00%)
  2 (2.00%) high mild
  5 (5.00%) high severe

growing_tree_method_random/generate_100_x_100
                        time:   [1.5472 ms 1.5485 ms 1.5499 ms]
Found 6 outliers among 100 measurements (6.00%)
  1 (1.00%) high mild
  5 (5.00%) high severe

growing_tree_method_oldest/generate_10_x_10
                        time:   [11.401 µs 11.410 µs 11.427 µs]
Found 8 outliers among 100 measurements (8.00%)
  1 (1.00%) high mild
  7 (7.00%) high severe

growing_tree_method_oldest/generate_100_x_100
                        time:   [1.1748 ms 1.1757 ms 1.1765 ms]
Found 3 outliers among 100 measurements (3.00%)
  1 (1.00%) high mild
  2 (2.00%) high severe

growing_tree_method_newest/generate_10_x_10
                        time:   [10.606 µs 10.612 µs 10.620 µs]
Found 9 outliers among 100 measurements (9.00%)
  4 (4.00%) high mild
  5 (5.00%) high severe

growing_tree_method_newest/generate_100_x_100
                        time:   [989.21 µs 989.49 µs 989.81 µs]
Found 13 outliers among 100 measurements (13.00%)
  1 (1.00%) low mild
  4 (4.00%) high mild
  8 (8.00%) high severe

growing_tree_method_middle/generate_10_x_10
                        time:   [11.400 µs 11.409 µs 11.418 µs]
Found 1 outliers among 100 measurements (1.00%)
  1 (1.00%) high mild

growing_tree_method_middle/generate_100_x_100
                        time:   [1.2195 ms 1.2302 ms 1.2390 ms]

hunt_and_kill/generate_10_x_10
                        time:   [5.1945 µs 5.2087 µs 5.2217 µs]

hunt_and_kill/generate_100_x_100
                        time:   [533.06 µs 533.83 µs 534.64 µs]
Found 3 outliers among 100 measurements (3.00%)
  1 (1.00%) low severe
  1 (1.00%) low mild
  1 (1.00%) high mild

kruskal/generate_10_x_10
                        time:   [10.249 µs 10.275 µs 10.305 µs]
Found 7 outliers among 100 measurements (7.00%)
  5 (5.00%) high mild
  2 (2.00%) high severe

kruskal/generate_100_x_100
                        time:   [46.120 ms 46.429 ms 46.738 ms]

prim/generate_10_x_10   time:   [9.9880 µs 9.9983 µs 10.009 µs]
Found 13 outliers among 100 measurements (13.00%)
  4 (4.00%) low mild
  4 (4.00%) high mild
  5 (5.00%) high severe

prim/generate_100_x_100 time:   [2.6866 ms 2.6930 ms 2.6995 ms]
Found 7 outliers among 100 measurements (7.00%)
  2 (2.00%) low mild
  3 (3.00%) high mild
  2 (2.00%) high severe

recursive_backtracking/generate_10_x_10
                        time:   [6.0566 µs 6.0581 µs 6.0598 µs]
Found 8 outliers among 100 measurements (8.00%)
  3 (3.00%) high mild
  5 (5.00%) high severe

recursive_backtracking/generate_100_x_100
                        time:   [605.06 µs 610.69 µs 622.53 µs]
Found 13 outliers among 100 measurements (13.00%)
  5 (5.00%) high mild
  8 (8.00%) high severe

recursive_division/generate_10_x_10
                        time:   [1.7657 µs 1.7678 µs 1.7698 µs]
Found 1 outliers among 100 measurements (1.00%)
  1 (1.00%) high mild

recursive_division/generate_100_x_100
                        time:   [168.76 µs 169.02 µs 169.35 µs]
Found 9 outliers among 100 measurements (9.00%)
  1 (1.00%) high mild
  8 (8.00%) high severe

sidewinder/generate_10_x_10
                        time:   [1.6033 µs 1.6047 µs 1.6060 µs]
Found 14 outliers among 100 measurements (14.00%)
  2 (2.00%) high mild
  12 (12.00%) high severe

sidewinder/generate_100_x_100
                        time:   [143.58 µs 144.45 µs 145.82 µs]
Found 3 outliers among 100 measurements (3.00%)
  3 (3.00%) high severe

game_map/format_10_x_10 time:   [317.58 µs 319.06 µs 320.62 µs]
Found 13 outliers among 100 measurements (13.00%)
  2 (2.00%) low severe
  7 (7.00%) low mild
  3 (3.00%) high mild
  1 (1.00%) high severe

game_map/format_100_x_100
                        time:   [3.3593 ms 3.3636 ms 3.3684 ms]
Found 5 outliers among 100 measurements (5.00%)
  1 (1.00%) high mild
  4 (4.00%) high severe

ascii_broad/format_10_x_10
                        time:   [301.03 µs 302.43 µs 303.76 µs]
Found 7 outliers among 100 measurements (7.00%)
  3 (3.00%) low mild
  1 (1.00%) high mild
  3 (3.00%) high severe

ascii_broad/format_100_x_100
                        time:   [999.95 µs 1.0017 ms 1.0035 ms]
Found 8 outliers among 100 measurements (8.00%)
  1 (1.00%) low mild
  4 (4.00%) high mild
  3 (3.00%) high severe

ascii_narrow/format_10_x_10
                        time:   [277.69 µs 280.77 µs 284.79 µs]
Found 16 outliers among 100 measurements (16.00%)
  4 (4.00%) low mild
  5 (5.00%) high mild
  7 (7.00%) high severe

ascii_narrow/format_100_x_100
                        time:   [938.55 µs 940.03 µs 941.44 µs]
Found 7 outliers among 100 measurements (7.00%)
  3 (3.00%) low mild
  3 (3.00%) high mild
  1 (1.00%) high severe

image/format_10_x_10    time:   [9.0581 ms 9.0701 ms 9.0826 ms]
Found 4 outliers among 100 measurements (4.00%)
  1 (1.00%) low mild
  3 (3.00%) high mild

image/format_100_x_100  time:   [770.69 ms 773.80 ms 778.30 ms]
Found 17 outliers among 100 measurements (17.00%)
  15 (15.00%) high mild
  2 (2.00%) high severe
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
