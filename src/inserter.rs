use regex::Regex;
use std::collections::HashMap;

pub fn insert_summaries(input: &str, version_to_summary: &HashMap<String, String>) -> String {
    // For example: ## [0.5.0] - 2026-02-17
    let header_regex = Regex::new(r"^## \[(\d+\.\d+\.\d+)\] - (\d{4}-\d{2}-\d{2})$")
        .expect("Failed to parse regex.");

    let input_lines = input.lines();
    let mut output_lines: Vec<&str> = Vec::new();

    for line in input_lines {
        match header_regex.captures(line) {
            Some(captures) => {
                let version = captures[1].to_string();
                let summary = version_to_summary
                    .get(&version)
                    .expect("Orphan version without summary fuond in input.")
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
    output_lines.join("\n")
}
