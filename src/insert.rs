use std::collections::HashMap;

use anyhow::{Context, Result, anyhow};
use regex::Regex;

pub fn insert_summaries(
    input: &str,
    version_to_summary: &HashMap<String, String>,
) -> Result<String> {
    // For example: ## [0.5.0] - 2026-02-17
    let header_re = Regex::new(r"^## \[(?<version>\d+\.\d+\.\d+)\] - (\d{4}-\d{2}-\d{2})$")?;

    let input_lines = input.lines();
    let mut output_lines = Vec::new();

    for line in input_lines {
        match header_re.captures(line) {
            Some(captures) => {
                let version = captures
                    .name("version")
                    .with_context(|| anyhow!("failed to extract verion from line {}", line))?
                    .as_str();

                let summary = version_to_summary
                    .get(version)
                    .with_context(|| anyhow!("expected version {} has a summary section", version))?
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
