use assert_cmd::Command;

const CLI_HELP_STR: &str = "Rust library for generating and rendering mazes

Usage: knossos <COMMAND>

Commands:
  generate  Generates a maze
  help      Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version
";

const GENERATE_SHORT_HELP_STR: &str = "Generates a maze

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
          Bias to use for the \"Binary Tree\" algorithm [default: north-east] [possible values: north-west, north-east, south-west, south-east]
      --growing-method[=<GROWING_METHOD>]
          Growing method to use for the \"Growing Tree\" algorithm [default: newest] [possible values: newest, oldest, random, middle, newest50-random50, newest75-random25, newest25-random75]
  -h, --help
          Print help (see more with '--help')
";

const GENERATE_LONG_HELP_STR: &str = "Generates a maze

Usage: knossos generate [OPTIONS] <COMMAND>

Commands:
  ascii     Save to a text file with an ASCII representation of a maze
  game-map  Save to a text file as an ASCII game map for pseudo 3D games that use ray casting for modeling and rendering the map
  image     Save to PNG or JPG file
  help      Print this message or the help of the given subcommand(s)

Options:
  -A, --algorithm <ALGORITHM>
          Maze generation algorithm
          
          [default: recursive-backtracking]
          [possible values: aldous-broder, binary-tree, eller, growing-tree, hunt-and-kill, kruskal, prim, recursive-backtracking, recursive-division, sidewinder]

  -H, --height <HEIGHT>
          Grid height in a number of cells
          
          [default: 10]

  -W, --width <WIDTH>
          Grid width in a number of cells
          
          [default: 10]

      --seed <SEED>
          Seed value for deterministic generation (must be a valid u64)

      --bias[=<BIAS>]
          Bias to use for the \"Binary Tree\" algorithm
          
          [default: north-east]

          Possible values:
          - north-west: Produces two long corridors on the Northern and Western sides of the maze
          - north-east: Produces two long corridors on the Northern and Eastern sides of the maze
          - south-west: Produces two long corridors on the Southern and Western sides of the maze
          - south-east: Produces two long corridors on the Southern and Eastern sides of the maze

      --growing-method[=<GROWING_METHOD>]
          Growing method to use for the \"Growing Tree\" algorithm
          
          [default: newest]

          Possible values:
          - newest:            Selects the most recently added cell, thus imitating the recursive backtracker
          - oldest:            Selects the oldest added cell, thus generating an unchallenging maze with lots of long corridors
          - random:            Selects cells at random, thus getting Prim's algorithm behaviour
          - middle:            Selects a middle cell from the list of already added, but produces mazes similar to the ones created by the [Oldest](Method::Oldest) method
          - newest50-random50: A combination of the [Newest](Method::Newest) and [Random](Method::Random) methods with 50/50 split
          - newest75-random25: A combination of the [Newest](Method::Newest) and [Random](Method::Random) methods with 75/25 split
          - newest25-random75: A combination of the [Newest](Method::Newest) and [Random](Method::Random) methods with 25/75 split

  -h, --help
          Print help (see a summary with '-h')
";

const GENERATE_IMAGE_HELP_STR: &str = "Save to PNG or JPG file

Usage: knossos generate image [OPTIONS] --output-path <OUTPUT_PATH>

Options:
  -O, --output-path <OUTPUT_PATH>      Output path
      --wall-size <WALL_SIZE>          Wall size in pixels [default: 40]
      --passage-size <PASSAGE_SIZE>    Passage size in pixels [default: 40]
      --margin <MARGIN>                Size of the margin area that implies an empty space between an image borders and grid [default: 50]
      --passage-color <PASSAGE_COLOR>  Color of passages [default: #ffffff]
      --wall-color <WALL_COLOR>        Color of walls [default: #000000]
  -h, --help                           Print help
";

const GENERATE_ASCII_HELP_STR: &str = "Save to a text file with an ASCII representation of a maze

Usage: knossos generate ascii [OPTIONS] --output-path <OUTPUT_PATH>

Options:
  -O, --output-path <OUTPUT_PATH>    Output path
  -T, --output-type[=<OUTPUT_TYPE>]  Output type [default: narrow] [possible values: narrow, broad]
  -h, --help                         Print help
";

const GENERATE_GAME_MAP_HELP_STR: &str = "Save to a text file as an ASCII game map for pseudo 3D games that use ray casting for modeling and rendering the map

Usage: knossos generate game-map [OPTIONS] --output-path <OUTPUT_PATH>

Options:
  -O, --output-path <OUTPUT_PATH>  Output path
      --span <SPAN>                Distance between any two walls [default: 3]
      --passage <PASSAGE>          ASCII character for a passage [default: .]
      --wall <WALL>                ASCII character for a wall [default: #]
      --with-start-goal            With start \"S\" and goal \"G\" points randomly spawned on the borders
  -h, --help                       Print help
";

#[test]
fn cli_help() {
    let mut cmd = Command::cargo_bin(assert_cmd::crate_name!()).unwrap();
    cmd.arg("--help").assert().success().stdout(CLI_HELP_STR);
}

#[test]
fn command_generate_short_help() {
    let mut cmd = Command::cargo_bin(assert_cmd::crate_name!()).unwrap();
    cmd.arg("generate")
        .arg("-h")
        .assert()
        .success()
        .stdout(GENERATE_SHORT_HELP_STR);
}

#[test]
fn command_generate_long_help() {
    let mut cmd = Command::cargo_bin(assert_cmd::crate_name!()).unwrap();
    cmd.arg("generate")
        .arg("--help")
        .assert()
        .success()
        .stdout(GENERATE_LONG_HELP_STR);
}

#[test]
fn command_generate_image_help() {
    let mut cmd = Command::cargo_bin(assert_cmd::crate_name!()).unwrap();
    cmd.arg("generate")
        .arg("image")
        .arg("--help")
        .assert()
        .success()
        .stdout(GENERATE_IMAGE_HELP_STR);
}

#[test]
fn command_generate_ascii_help() {
    let mut cmd = Command::cargo_bin(assert_cmd::crate_name!()).unwrap();
    cmd.arg("generate")
        .arg("ascii")
        .arg("--help")
        .assert()
        .success()
        .stdout(GENERATE_ASCII_HELP_STR);
}

#[test]
fn command_generate_game_map_help() {
    let mut cmd = Command::cargo_bin(assert_cmd::crate_name!()).unwrap();
    cmd.arg("generate")
        .arg("game-map")
        .arg("--help")
        .assert()
        .success()
        .stdout(GENERATE_GAME_MAP_HELP_STR);
}
