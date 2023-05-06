use knossos::maze::*;
use assert_fs::fixture::TempDir;

macro_rules! maze {
    ($algo:expr) => {
        OrthogonalMazeBuilder::new()
            .algorithm(Box::new($algo))
            .build()
    };

    () => {
        OrthogonalMazeBuilder::new().build()
    };
}

#[test]
fn build_valid_maze_with_default_params() {
    let maze = OrthogonalMazeBuilder::new().build();
    assert!(maze.is_valid());
}

#[test]
fn build_valid_maze_with_custom_params() {
    let maze = OrthogonalMazeBuilder::new()
        .height(10)
        .width(20)
        .algorithm(Box::new(Kruskal))
        .build();

    assert!(maze.is_valid());
}

#[test]
fn build_valid_maze_with_aldou_broder_algorithm() {
    assert!(maze!(AldousBroder).is_valid());
}

#[test]
fn build_valid_maze_with_binary_tree_algorithm() {
    assert!(maze!(BinaryTree::new(Bias::NorthWest)).is_valid());
    assert!(maze!(BinaryTree::new(Bias::SouthWest)).is_valid());
    assert!(maze!(BinaryTree::new(Bias::NorthEast)).is_valid());
    assert!(maze!(BinaryTree::new(Bias::SouthEast)).is_valid());
}

#[test]
fn build_valid_maze_with_eller_algorithm() {
    assert!(maze!(Eller).is_valid());
}

#[test]
fn build_valid_maze_with_growing_tree_algorithm() {
    assert!(maze!(GrowingTree::new(Method::Newest)).is_valid());
    assert!(maze!(GrowingTree::new(Method::Oldest)).is_valid());
    assert!(maze!(GrowingTree::new(Method::Middle)).is_valid());
    assert!(maze!(GrowingTree::new(Method::Random)).is_valid());
    assert!(maze!(GrowingTree::new(Method::Newest25Random75)).is_valid());
    assert!(maze!(GrowingTree::new(Method::Newest50Random50)).is_valid());
    assert!(maze!(GrowingTree::new(Method::Newest75Random25)).is_valid());
}

#[test]
fn build_valid_maze_with_hunt_and_kill_algorithm() {
    assert!(maze!(HuntAndKill::new()).is_valid());
}

#[test]
fn build_valid_maze_with_kruskal_algorithm() {
    assert!(maze!(Kruskal).is_valid());
}

#[test]
fn build_valid_maze_with_prim_algorithm() {
    assert!(maze!(Prim::new()).is_valid());
}

#[test]
fn build_valid_maze_with_recursive_backtracking_algorithm() {
    assert!(maze!(RecursiveBacktracking).is_valid());
}

#[test]
fn build_valid_maze_with_recursive_division_algorithm() {
    assert!(maze!(RecursiveDivision).is_valid());
}

#[test]
fn build_valid_maze_with_sidewinder_algorithm() {
    assert!(maze!(Sidewinder).is_valid());
}

macro_rules! to_absolute_path {
    ($path:expr) => {
        std::env::current_dir().unwrap().join($path).display()
    };
}

macro_rules! assert_save_maze {
    ($path:expr, $formatter:expr, $expected:expr) => {
        let maze = maze!();
        let result = maze.save($path, $formatter);
        assert_eq!($expected, result.unwrap());
    };
}

macro_rules! assert_save_maze_error {
    ($path:expr, $formatter:expr, $expected:expr) => {
        let maze = maze!();
        let result = maze.save($path, $formatter);
        assert_eq!($expected, result.unwrap_err().reason);
    };
}

#[test]
fn save_maze_as_ascii() {
    let output_dir = TempDir::new().unwrap();
    let file_path = format!("{}/maze.png", output_dir.path().display());
    let expected = format!(
        "Maze was successfully written to a file: {}",
        to_absolute_path!(&file_path)
    );
    assert_save_maze!(&file_path, AsciiNarrow, expected);
}

#[test]
#[cfg(target_os = "linux")]
fn save_maze_as_ascii_returns_error() {
    let expected = format!(
        "Couldn't create {}: Is a directory (os error 21)",
        to_absolute_path!("this is not valid path/")
    );

    assert_save_maze_error!("this is not valid path/", AsciiNarrow, expected);
}

#[test]
#[cfg(target_os = "macos")]
fn save_maze_as_ascii_returns_error() {
    let expected = format!(
        "Couldn't create {}: No such file or directory (os error 2)",
        to_absolute_path!("this is not valid path/")
    );

    assert_save_maze_error!("this is not valid path/", AsciiNarrow, expected);
}

#[test]
fn save_maze_as_game_map() {
    let output_dir = TempDir::new().unwrap();
    let file_path = format!("{}/maze.png", output_dir.path().display());
    let expected = format!(
        "Maze was successfully written to a file: {}",
        to_absolute_path!(&file_path)
    );
    assert_save_maze!(&file_path, GameMap::new(), expected);
}

#[test]
#[cfg(target_os = "linux")]
fn save_maze_as_game_map_returns_error() {
    let expected = format!(
        "Couldn't create {}: Is a directory (os error 21)",
        to_absolute_path!("this is not valid path/")
    );

    assert_save_maze_error!("this is not valid path/", GameMap::new(), expected);
}

#[test]
#[cfg(target_os = "macos")]
fn save_maze_as_game_map_returns_error() {
    let expected = format!(
        "Couldn't create {}: No such file or directory (os error 2)",
        to_absolute_path!("this is not valid path/")
    );

    assert_save_maze_error!("this is not valid path/", GameMap::new(), expected);
}

#[test]
fn save_maze_as_png() {
    let output_dir = TempDir::new().unwrap();
    let file_path = format!("{}/maze.png", output_dir.path().display());
    let expected = format!("Maze was successfully saved as an image: {}", &file_path);
    assert_save_maze!(&file_path, Image::new(), expected);
}

#[test]
fn save_maze_as_png_returns_error() {
    let expected = "The image format could not be determined".to_string();
    assert_save_maze_error!("this is not valid path/", Image::new(), expected);
}
