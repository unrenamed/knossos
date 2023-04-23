use assert_cmd::Command;
use assert_fs::fixture::TempDir;

#[test]
fn image_save_success() {
    let output_dir = TempDir::new().unwrap();
    let file_path = format!("{}/maze.png", output_dir.path().display());
    let expected = format!("Maze was successfully saved as an image: {}\n", &file_path);

    let mut cmd = Command::cargo_bin(assert_cmd::crate_name!()).unwrap();
    cmd.args(["generate", "image", "--output-path", &file_path])
        .assert()
        .success()
        .stdout(expected);
}

#[test]
fn ascii_save_success() {
    let output_dir = TempDir::new().unwrap();
    let file_path = format!("{}/maze.txt", output_dir.path().display());
    let expected = format!("Maze was successfully written to a file: {}\n", file_path);

    let mut cmd = Command::cargo_bin(assert_cmd::crate_name!()).unwrap();
    cmd.args(["generate", "ascii", "--output-path", &file_path])
        .assert()
        .success()
        .stdout(expected);
}

#[test]
fn game_map_save_success() {
    let output_dir = TempDir::new().unwrap();
    let file_path = format!("{}/maze.txt", output_dir.path().display());
    let expected = format!("Maze was successfully written to a file: {}\n", file_path);

    let mut cmd = Command::cargo_bin(assert_cmd::crate_name!()).unwrap();
    cmd.args(["generate", "game-map", "--output-path", &file_path])
        .assert()
        .success()
        .stdout(expected);
}
