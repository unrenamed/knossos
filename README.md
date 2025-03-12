[![Latest Version](https://img.shields.io/crates/v/knossos)](https://crates.io/crates/knossos)
[![License:Apache](https://img.shields.io/badge/License-Apache_2.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)
[![Build Status](https://github.com/unrenamed/knossos/actions/workflows/build.yml/badge.svg?branch=main)](https://github.com/unrenamed/knossos/actions/workflows/build.yml)
[![Coverage Status](https://coveralls.io/repos/github/unrenamed/knossos/badge.svg)](https://coveralls.io/github/unrenamed/knossos)

<p align="center">
  <img src="assets/daedalus.png?raw=true" width="400" height="400">
</p>

# Knossos

Knossos is a simple and flexible Rust library and CLI tool for generating mazes using various algorithms. It includes built-in utilities for rendering mazes as ASCII, images, or game maps and supports saving them in multiple formats.

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
knossos = "1.2.0"
```

## Usage

Knossos is designed to be super easy and convenient to use. Here are some usage examples of how to generate, display and save mazes:

### Generate with Default Parameters

```rust,no_run
use knossos::maze::*;

let maze = OrthogonalMazeBuilder::new().build();
```

### Generate with Custom Parameters

```rust,no_run
use knossos::maze::*;

let maze = OrthogonalMazeBuilder::new()
 .height(10)
 .width(10)
 .algorithm(Box::new(GrowingTree::new(Method::Random)))
 .build();
```

### Display Mazes

```rust,no_run
use knossos::maze::*;

let maze = OrthogonalMazeBuilder::new().build();
println!("{}", &maze);
```

### Save to File

```rust,no_run
use knossos::maze::*;

let maze = OrthogonalMazeBuilder::new().build();

// Save as ASCII text
maze.save("output/maze.txt", AsciiNarrow).unwrap();
// Save as a game map (with adjustable span size)
maze.save("output/maze_game_map.txt", GameMap::new().span(3)).unwrap();
// Save as a PNG image (adjusting wall and passage sizes)
maze.save("output/maze.png", Image::new().wall(10).passage(30)).unwrap();
```

### Format for Further Processing or Logging

```rust,no_run
use knossos::maze::*;

let maze = OrthogonalMazeBuilder::new().build();

// Convert to ASCII text
let ascii = maze.format(AsciiNarrow).into_inner();
// Convert to a game map
let game_map = maze.format(GameMap::new()).into_inner();
// Convert to an RGB image buffer
let rgb_image = maze.format(Image::new().wall(10).passage(30)).into_inner();
```

You can find more examples in the [examples](examples) directory. To run the example:

```bash
cargo run --example mazes
```

### Seeding for Deterministic Mazes

By default, each generated maze is randomized, producing a different layout every time. However,
you can use a seed value to ensure that the same maze is generated consistently across runs.
This is useful for debugging, testing, or sharing the exact same maze with others.

```rust,no_run
use knossos::maze::*;

// Generate a maze with a fixed seed
let maze = OrthogonalMazeBuilder::new().seed(Some(40)).build();
```

Passing `None` as the seed (or omitting the `.seed()` method) will result in a random maze each time.

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
aldous_broder/generate_10_x_10
                        time:   [43.817 µs 43.930 µs 44.047 µs]
Found 2 outliers among 100 measurements (2.00%)
  1 (1.00%) high mild
  1 (1.00%) high severe

aldous_broder/generate_100_x_100
                        time:   [13.019 ms 13.353 ms 13.691 ms]

binary_tree/generate_10_x_10
                        time:   [4.0313 µs 4.0335 µs 4.0364 µs]
Found 4 outliers among 100 measurements (4.00%)
  4 (4.00%) high severe

binary_tree/generate_100_x_100
                        time:   [396.58 µs 396.83 µs 397.15 µs]
Found 9 outliers among 100 measurements (9.00%)
  1 (1.00%) high mild
  8 (8.00%) high severe

sidewinder/generate_10_x_10
                        time:   [1.6094 µs 1.6107 µs 1.6122 µs]
Found 10 outliers among 100 measurements (10.00%)
  4 (4.00%) high mild
  6 (6.00%) high severe

sidewinder/generate_100_x_100
                        time:   [144.00 µs 144.46 µs 145.25 µs]
Found 12 outliers among 100 measurements (12.00%)
  7 (7.00%) high mild
  5 (5.00%) high severe

growing_tree_method_oldest/generate_10_x_10
                        time:   [11.093 µs 11.102 µs 11.111 µs]
Found 7 outliers among 100 measurements (7.00%)
  3 (3.00%) high mild
  4 (4.00%) high severe

growing_tree_method_oldest/generate_100_x_100
                        time:   [1.1500 ms 1.1513 ms 1.1528 ms]
Found 7 outliers among 100 measurements (7.00%)
  4 (4.00%) high mild
  3 (3.00%) high severe

growing_tree_method_newest/generate_10_x_10
                        time:   [10.468 µs 10.475 µs 10.483 µs]
Found 10 outliers among 100 measurements (10.00%)
  3 (3.00%) high mild
  7 (7.00%) high severe

growing_tree_method_newest/generate_100_x_100
                        time:   [996.60 µs 1.0059 ms 1.0166 ms]
Found 7 outliers among 100 measurements (7.00%)
  3 (3.00%) high mild
  4 (4.00%) high severe

growing_tree_method_middle/generate_10_x_10
                        time:   [11.239 µs 11.249 µs 11.260 µs]
Found 3 outliers among 100 measurements (3.00%)
  2 (2.00%) high mild
  1 (1.00%) high severe

growing_tree_method_middle/generate_100_x_100
                        time:   [1.1787 ms 1.1901 ms 1.2028 ms]
Found 8 outliers among 100 measurements (8.00%)
  2 (2.00%) high mild
  6 (6.00%) high severe

growing_tree_method_random/generate_10_x_10
                        time:   [12.823 µs 12.937 µs 13.075 µs]
Found 5 outliers among 100 measurements (5.00%)
  2 (2.00%) high mild
  3 (3.00%) high severe

growing_tree_method_random/generate_100_x_100
                        time:   [1.5642 ms 1.5756 ms 1.5882 ms]
Found 7 outliers among 100 measurements (7.00%)
  3 (3.00%) high mild
  4 (4.00%) high severe

kruskal/generate_10_x_10
                        time:   [10.254 µs 10.359 µs 10.506 µs]
Found 5 outliers among 100 measurements (5.00%)
  2 (2.00%) high mild
  3 (3.00%) high severe

kruskal/generate_100_x_100
                        time:   [46.782 ms 47.199 ms 47.617 ms]

prim/generate_10_x_10   time:   [10.027 µs 10.035 µs 10.044 µs]
Found 2 outliers among 100 measurements (2.00%)
  1 (1.00%) high mild
  1 (1.00%) high severe

prim/generate_100_x_100 time:   [2.6863 ms 2.6995 ms 2.7130 ms]
Found 1 outliers among 100 measurements (1.00%)
  1 (1.00%) high mild

eller/generate_10_x_10  time:   [23.493 µs 23.535 µs 23.589 µs]

eller/generate_100_x_100
                        time:   [2.2557 ms 2.2606 ms 2.2661 ms]
Found 2 outliers among 100 measurements (2.00%)
  1 (1.00%) high mild
  1 (1.00%) high severe

hunt_and_kill/generate_10_x_10
                        time:   [4.8662 µs 4.8699 µs 4.8745 µs]
Found 3 outliers among 100 measurements (3.00%)
  1 (1.00%) high mild
  2 (2.00%) high severe

hunt_and_kill/generate_100_x_100
                        time:   [526.59 µs 527.84 µs 529.21 µs]
Found 3 outliers among 100 measurements (3.00%)
  3 (3.00%) high mild

recursive_backtracking/generate_10_x_10
                        time:   [6.0658 µs 6.0727 µs 6.0817 µs]
Found 3 outliers among 100 measurements (3.00%)
  2 (2.00%) high mild
  1 (1.00%) high severe

recursive_backtracking/generate_100_x_100
                        time:   [603.01 µs 604.11 µs 605.67 µs]
Found 3 outliers among 100 measurements (3.00%)
  1 (1.00%) high mild
  2 (2.00%) high severe

recursive_division/generate_10_x_10
                        time:   [1.7652 µs 1.7673 µs 1.7699 µs]
Found 3 outliers among 100 measurements (3.00%)
  3 (3.00%) high mild

recursive_division/generate_100_x_100
                        time:   [168.23 µs 168.41 µs 168.64 µs]
Found 3 outliers among 100 measurements (3.00%)
  2 (2.00%) high mild
  1 (1.00%) high severe

ascii_narrow/format_10_x_10
                        time:   [257.18 µs 260.38 µs 264.60 µs]
Found 9 outliers among 100 measurements (9.00%)
  2 (2.00%) low mild
  3 (3.00%) high mild
  4 (4.00%) high severe

ascii_narrow/format_100_x_100
                        time:   [904.01 µs 905.75 µs 908.92 µs]
Found 8 outliers among 100 measurements (8.00%)
  2 (2.00%) high mild
  6 (6.00%) high severe

ascii_broad/format_10_x_10
                        time:   [259.78 µs 263.12 µs 268.50 µs]
Found 3 outliers among 100 measurements (3.00%)
  2 (2.00%) high mild
  1 (1.00%) high severe

ascii_broad/format_100_x_100
                        time:   [976.19 µs 979.58 µs 985.36 µs]
Found 9 outliers among 100 measurements (9.00%)
  1 (1.00%) low mild
  4 (4.00%) high mild
  4 (4.00%) high severe

game_map/format_10_x_10 time:   [281.71 µs 288.31 µs 301.60 µs]
Found 7 outliers among 100 measurements (7.00%)
  1 (1.00%) low mild
  3 (3.00%) high mild
  3 (3.00%) high severe

game_map/format_100_x_100
                        time:   [3.2842 ms 3.2915 ms 3.2996 ms]
Found 10 outliers among 100 measurements (10.00%)
  8 (8.00%) high mild
  2 (2.00%) high severe

image/format_10_x_10    time:   [8.8747 ms 8.8985 ms 8.9268 ms]
Found 6 outliers among 100 measurements (6.00%)
  3 (3.00%) high mild
  3 (3.00%) high severe

image/format_100_x_100  time:   [777.22 ms 777.81 ms 778.46 ms]
Found 6 outliers among 100 measurements (6.00%)
  1 (1.00%) high mild
  5 (5.00%) high severe
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
knossos 1.2.0
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
      --seed <SEED>
          Seed value for deterministic generation (must be a valid u64)
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
