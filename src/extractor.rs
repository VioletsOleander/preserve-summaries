use std::collections::HashMap;

use anyhow::{Context, Result};
use regex::Regex;

#[derive(Debug)]
struct SummaryIndex {
    range: (usize, usize),
    version: String,
}

impl SummaryIndex {
    pub fn new(range: (usize, usize), version: String) -> Self {
        SummaryIndex { range, version }
    }
}

/// If the CHANGELOG.md always follow a structure like:
///
/// ```markdown
/// ## [0.5.0] - 2026-02-17
///
/// ### Summary
///
/// There are some contents here.
///
/// ### Other subheaders
/// ```
/// The extraction should work.
pub fn extract_summaries(file: &str) -> Result<HashMap<String, String>> {
    // For example: ## [0.5.0] - 2026-02-17
    let header_regex = Regex::new(r"^## \[(\d+\.\d+\.\d+)\] - (\d{4}-\d{2}-\d{2})$")?;
    // For example: ### Feature
    let subheader_regex = Regex::new(r"^### ([A-Z][a-z]+)")?;

    let lines: Vec<_> = file.lines().collect();
    let mut summary_indices = Vec::new();

    for (idx, line) in lines.iter().enumerate() {
        match header_regex.captures(line) {
            Some(captures) => {
                let version = captures[1].to_string();
                summary_indices.push(SummaryIndex::new((0, 0), version));
            }
            None => match subheader_regex.captures(line) {
                Some(captures) => {
                    let subheader = captures[1].to_string();
                    let current_index = summary_indices
                        .last_mut()
                        .context("found orphan subheader in CHANGELOG.md")?;

                    if subheader == "Summary" {
                        // Start index
                        current_index.range.0 = idx;
                        current_index.range.1 = idx;
                    } else if current_index.range.0 == current_index.range.1 {
                        // End index
                        current_index.range.1 = idx;
                    }
                }
                None => {
                    continue;
                }
            },
        }
    }

    let mut version_to_summary = HashMap::new();

    for summary_index in summary_indices {
        let version = summary_index.version;
        let range = summary_index.range;

        let summary = lines[range.0..range.1].join("\n");

        version_to_summary.insert(version, summary);
    }

    Ok(version_to_summary)
}
