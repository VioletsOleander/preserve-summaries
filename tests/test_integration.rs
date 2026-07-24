use assert_cmd::{Command, pkg_name};
use pretty_assertions::assert_eq;
use std::fs;

#[test]
pub fn test_integration() {
    let command_dir = fs::canonicalize("./tests/").expect("Failed to parse command dir.");
    let input = read_file("./tests/test_input.md");
    let output = read_file("./tests/test_output.md");

    let mut cmd = Command::cargo_bin(pkg_name!()).expect("Failed to create command.");

    let assertion = cmd.current_dir(command_dir).write_stdin(input).assert();

    let stdout = assertion.get_output().stdout.clone();
    assert_eq!(output, String::from_utf8(stdout).unwrap());
}

fn read_file(path: &str) -> String {
    let p = fs::canonicalize(path).expect(&format!("Failed to canonicalize path {}.", path));
    fs::read_to_string(p).expect(&format!("Failed to read file"))
}
