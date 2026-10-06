use std::collections::HashMap;

use anyhow::{Context, Result, anyhow};
use regex::Regex;

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
///
/// The extraction should work.
pub fn extract_summaries(file: &str) -> Result<HashMap<String, String>> {
    // For example: ## [0.5.0] - 2026-02-17
    let header_re = Regex::new(r"^## \[(?<version>\d+\.\d+\.\d+)\] - (\d{4}-\d{2}-\d{2})$")?;
    // For example: ### Feature
    let subheader_re = Regex::new(r"^### (?<subheader>[A-Z][a-z]+)")?;

    let lines: Vec<_> = file.lines().collect();
    let mut version_to_summary = HashMap::new();

    let mut version: Option<&str> = None;
    let mut start_idx: Option<usize> = None;

    for (idx, line) in lines.iter().enumerate() {
        if let Some(captures) = header_re.captures(line) {
            if version.is_some() {
                return Err(anyhow!("expectd version to be None, in line {}", line));
            }

            version = Some(
                captures
                    .name("version")
                    .with_context(|| anyhow!("failed to capture version in line {}", line))?
                    .as_str(),
            );

            continue;
        }

        if let Some(captures) = subheader_re.captures(line) {
            let subheader = captures
                .name("subheader")
                .with_context(|| anyhow!("failed to capture subheader in line {}", line))?
                .as_str()
                .to_string();

            if let Some(ver) = version {
                match start_idx {
                    Some(start) => {
                        let summary = lines[start..idx].join("\n");

                        version_to_summary.insert(ver.to_string(), summary);

                        start_idx = None;
                        version = None;
                    }
                    None => {
                        if subheader == "Summary" {
                            start_idx = Some(idx)
                        } else {
                            return Err(anyhow!(
                                "expected summmary subheader be the first subheader"
                            ));
                        }
                    }
                }
            };
        }
    }

    Ok(version_to_summary)
}
