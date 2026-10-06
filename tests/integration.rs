use std::fs;

use anyhow::Result;
use assert_cmd::{Command, pkg_name};
use pretty_assertions::assert_eq;

#[test]
fn integration() -> Result<()> {
    let command_dir = fs::canonicalize("./tests/")?;
    let input = read_file("./tests/test_input.md")?;
    let expected = read_file("./tests/test_output.md")?;

    let assertion = Command::cargo_bin(pkg_name!())?
        .current_dir(command_dir)
        .write_stdin(input)
        .assert()
        .success();

    let output = assertion.get_output();

    // Make sure dbg! is not captured.
    let stderr = String::from_utf8(output.stderr.clone()).unwrap();
    if !stderr.is_empty() {
        eprint!("{stderr}");
    }

    let result = String::from_utf8(output.stdout.clone()).unwrap();

    assert_eq!(result, expected);

    Ok(())
}

fn read_file(path: &str) -> Result<String> {
    let p = fs::canonicalize(path)?;
    let s = fs::read_to_string(p)?;

    Ok(s)
}
