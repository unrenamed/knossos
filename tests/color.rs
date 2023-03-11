use assert_cmd::Command;

struct TestCli {
    cmd: Command,
}

impl TestCli {
    fn new() -> Self {
        let mut cmd = Command::cargo_bin(assert_cmd::crate_name!()).unwrap();

        cmd.arg("generate")
            .arg("image")
            .arg("--output-path")
            .arg("./tests/output");

        Self { cmd }
    }
}

const WALL_COLOR_LEN_ERR: &str = "error: invalid value 'ff' for '--wall-color <WALL_COLOR>': Expected a 6 character color value in hex, but got: \"ff\"

For more information, try '--help'.
";

const WALL_COLOR_RGB_ERR: &str =
    "error: invalid value 'ZZZZZZ' for '--wall-color <WALL_COLOR>': invalid digit found in string

For more information, try '--help'.
";

#[test]
fn invalid_wall_color_length() {
    let mut cmd = TestCli::new().cmd;
    cmd.arg("--wall-color")
        .arg("ff")
        .assert()
        .failure()
        .stderr(WALL_COLOR_LEN_ERR);
}

#[test]
fn invalid_wall_color_rgb_value() {
    let mut cmd = TestCli::new().cmd;
    cmd.arg("--wall-color")
        .arg("ZZZZZZ")
        .assert()
        .failure()
        .stderr(WALL_COLOR_RGB_ERR);
}
