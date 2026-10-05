use std::collections::HashMap;

use anyhow::{Context, Result};
use regex::Regex;

pub fn insert_summaries(
    input: &str,
    version_to_summary: &HashMap<String, String>,
) -> Result<String> {
    // For example: ## [0.5.0] - 2026-02-17
    let header_regex = Regex::new(r"^## \[(\d+\.\d+\.\d+)\] - (\d{4}-\d{2}-\d{2})$")?;

    let input_lines = input.lines();
    let mut output_lines: Vec<&str> = Vec::new();

    for line in input_lines {
        match header_regex.captures(line) {
            Some(captures) => {
                let version = captures[1].to_string();
                let summary = version_to_summary
                    .get(&version)
                    .context("orphan version without summary found in input")?
                    .trim_end(); // Trim trailing '\n'

                output_lines.push(line);
                output_lines.push("");
                output_lines.push(summary);
            }
            None => {
                output_lines.push(line);
            }
        }
    }

    output_lines.push(""); // Make sure the last line also ends with a '\n'
    Ok(output_lines.join("\n"))
}
