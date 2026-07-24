use anyhow::Result;
use std::fs::File;
use std::io::{self, Read};

pub fn read_input() -> Result<String> {
    let mut stdin = io::stdin();
    let mut buf = String::new();

    stdin.read_to_string(&mut buf)?;
    Ok(buf)
}

pub fn read_file() -> Result<String> {
    let mut f = File::open("CHANGELOG.md").expect("Failed to open CHANGELOG.md");
    let mut buf = String::new();

    f.read_to_string(&mut buf)?;
    Ok(buf)
}
