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
knossos = "1.0.0"
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

### Format a maze for further processing, or logging

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

Knossos uses [Criterion.rs](https://github.com/bheisler/criterion.rs) for statistical benchmarking.

### Running Benchmarks

To run benchmarks locally:

```bash
cargo bench
```

This generates both terminal output and an HTML report in:

📂 `target/criterion/report/index.html` (Open in a browser for graphs and analysis)

### Summary of Recent Benchmarks

```bash
sidewinder/generate_10_x_10
                        time:   [1.6065 µs 1.6117 µs 1.6207 µs]
Found 9 outliers among 100 measurements (9.00%)
  5 (5.00%) high mild
  4 (4.00%) high severe

sidewinder/generate_100_x_100
                        time:   [143.83 µs 143.90 µs 144.01 µs]
Found 10 outliers among 100 measurements (10.00%)
  3 (3.00%) high mild
  7 (7.00%) high severe

binary_tree/generate_10_x_10
                        time:   [4.0141 µs 4.0169 µs 4.0201 µs]
Found 8 outliers among 100 measurements (8.00%)
  1 (1.00%) low mild
  4 (4.00%) high mild
  3 (3.00%) high severe

binary_tree/generate_100_x_100
                        time:   [395.34 µs 396.81 µs 399.11 µs]
Found 33 outliers among 100 measurements (33.00%)
  18 (18.00%) low severe
  3 (3.00%) low mild
  3 (3.00%) high mild
  9 (9.00%) high severe

recursive_division/generate_10_x_10
                        time:   [1.7687 µs 1.7706 µs 1.7725 µs]
Found 5 outliers among 100 measurements (5.00%)
  3 (3.00%) high mild
  2 (2.00%) high severe

recursive_division/generate_100_x_100
                        time:   [168.74 µs 168.88 µs 169.04 µs]
Found 8 outliers among 100 measurements (8.00%)
  1 (1.00%) low mild
  3 (3.00%) high mild
  4 (4.00%) high severe

growing_tree_method_oldest/generate_10_x_10
                        time:   [11.104 µs 11.111 µs 11.118 µs]
Found 8 outliers among 100 measurements (8.00%)
  7 (7.00%) high mild
  1 (1.00%) high severe

growing_tree_method_oldest/generate_100_x_100
                        time:   [1.1520 ms 1.1548 ms 1.1596 ms]
Found 3 outliers among 100 measurements (3.00%)
  1 (1.00%) high mild
  2 (2.00%) high severe

growing_tree_method_newest/generate_10_x_10
                        time:   [10.563 µs 10.569 µs 10.576 µs]
Found 1 outliers among 100 measurements (1.00%)
  1 (1.00%) high severe

growing_tree_method_newest/generate_100_x_100
                        time:   [984.81 µs 985.11 µs 985.46 µs]
Found 23 outliers among 100 measurements (23.00%)
  5 (5.00%) high mild
  18 (18.00%) high severe

growing_tree_method_middle/generate_10_x_10
                        time:   [11.302 µs 11.309 µs 11.316 µs]
Found 22 outliers among 100 measurements (22.00%)
  16 (16.00%) high mild
  6 (6.00%) high severe

growing_tree_method_middle/generate_100_x_100
                        time:   [1.1612 ms 1.1621 ms 1.1632 ms]
Found 7 outliers among 100 measurements (7.00%)
  1 (1.00%) low mild
  5 (5.00%) high mild
  1 (1.00%) high severe

growing_tree_method_random/generate_10_x_10
                        time:   [12.640 µs 12.647 µs 12.655 µs]
Found 2 outliers among 100 measurements (2.00%)
  1 (1.00%) high mild
  1 (1.00%) high severe

growing_tree_method_random/generate_100_x_100
                        time:   [1.5303 ms 1.5317 ms 1.5330 ms]
Found 12 outliers among 100 measurements (12.00%)
  6 (6.00%) high mild
  6 (6.00%) high severe

recursive_backtracking/generate_10_x_10
                        time:   [6.0741 µs 6.0873 µs 6.1105 µs]
Found 2 outliers among 100 measurements (2.00%)
  2 (2.00%) high severe

recursive_backtracking/generate_100_x_100
                        time:   [602.83 µs 603.19 µs 603.66 µs]
Found 1 outliers among 100 measurements (1.00%)
  1 (1.00%) high severe

prim/generate_10_x_10   time:   [10.049 µs 10.057 µs 10.066 µs]
Found 1 outliers among 100 measurements (1.00%)
  1 (1.00%) high mild

prim/generate_100_x_100 time:   [2.7083 ms 2.7231 ms 2.7381 ms]
Found 1 outliers among 100 measurements (1.00%)
  1 (1.00%) high mild

prim/generate_10_x_10   time:   [10.073 µs 10.080 µs 10.089 µs]
Found 2 outliers among 100 measurements (2.00%)
  1 (1.00%) high mild
  1 (1.00%) high severe

prim/generate_100_x_100 time:   [2.7002 ms 2.7133 ms 2.7267 ms]
Found 1 outliers among 100 measurements (1.00%)
  1 (1.00%) high mild

prim/generate_10_x_10   time:   [10.070 µs 10.078 µs 10.088 µs]
Found 1 outliers among 100 measurements (1.00%)
  1 (1.00%) high severe

prim/generate_100_x_100 time:   [2.6964 ms 2.7104 ms 2.7250 ms]
Found 2 outliers among 100 measurements (2.00%)
  2 (2.00%) high mild

aldous_broder/generate_10_x_10
                        time:   [43.850 µs 43.980 µs 44.134 µs]
Found 10 outliers among 100 measurements (10.00%)
  4 (4.00%) low mild
  3 (3.00%) high mild
  3 (3.00%) high severe

aldous_broder/generate_100_x_100
                        time:   [12.997 ms 13.361 ms 13.736 ms]
Found 1 outliers among 100 measurements (1.00%)
  1 (1.00%) high mild

ascii_narrow/format_10_x_10
                        time:   [270.02 µs 289.97 µs 330.33 µs]
Found 4 outliers among 100 measurements (4.00%)
  4 (4.00%) high severe

ascii_narrow/format_100_x_100
                        time:   [921.11 µs 932.28 µs 946.76 µs]
Found 7 outliers among 100 measurements (7.00%)
  3 (3.00%) high mild
  4 (4.00%) high severe

ascii_broad/format_10_x_10
                        time:   [264.94 µs 267.00 µs 269.13 µs]
Found 5 outliers among 100 measurements (5.00%)
  2 (2.00%) high mild
  3 (3.00%) high severe

ascii_broad/format_100_x_100
                        time:   [971.27 µs 979.57 µs 995.75 µs]
Found 11 outliers among 100 measurements (11.00%)
  8 (8.00%) high mild
  3 (3.00%) high severe

game_map/format_10_x_10 time:   [315.49 µs 362.48 µs 418.47 µs]
Found 14 outliers among 100 measurements (14.00%)
  4 (4.00%) high mild
  10 (10.00%) high severe

game_map/format_100_x_100
                        time:   [3.6767 ms 3.7856 ms 3.9381 ms]
Found 14 outliers among 100 measurements (14.00%)
  6 (6.00%) high mild
  8 (8.00%) high severe

image/format_10_x_10    time:   [8.9147 ms 8.9758 ms 9.0558 ms]
Found 20 outliers among 100 measurements (20.00%)
  14 (14.00%) high mild
  6 (6.00%) high severe

image/format_100_x_100  time:   [779.00 ms 781.22 ms 783.80 ms]
Found 17 outliers among 100 measurements (17.00%)
  8 (8.00%) high mild
  9 (9.00%) high severe
```

_(Benchmarks were run on an Apple M1 Max, Rust 1.84.0, Criterion 0.5.1)_

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
knossos 1.0.0
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
