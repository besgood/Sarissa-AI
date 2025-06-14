//! Module to revalidate actions suggested by the AI or correlation logic

use crate::analysis::correlation::CorrelatedFinding;
use std::process::Command;

/// Wraps a correlated finding with optional retest result
#[derive(Debug, Clone)]
pub struct RetestResult {
    pub finding: CorrelatedFinding,
    pub success: bool,
    pub output_snippet: String,
}

/// Try re-running simple commands based on tool logic
pub fn retest_suggestions(findings: &[CorrelatedFinding]) -> Vec<RetestResult> {
    let mut results = Vec::new();

    for f in findings {
        let mut cmd = Command::new("echo");
        let mut snippet = "dry run".to_string();
        let mut success = false;

        if f.suggested_tool == "ffuf" {
            cmd = Command::new("ffuf")
                .args(["-u", "http://target.com/FUZZ", "-w", "/usr/share/wordlists/dirb/common.txt"]);
        } else if f.suggested_tool == "sqlmap" {
            cmd = Command::new("sqlmap")
                .args(["-u", "http://target.com/test?id=1", "--batch"]);
        } else if f.suggested_tool == "nikto" {
            cmd = Command::new("nikto")
                .args(["-h", "http://target.com"]);
        }

        match cmd.output() {
            Ok(output) => {
                snippet = String::from_utf8_lossy(&output.stdout).lines().take(5).collect::<Vec<_>>().join("\n");
                success = output.status.success();
            }
            Err(_) => {
                snippet = "Command failed to run".to_string();
            }
        }

        results.push(RetestResult {
            finding: f.clone(),
            success,
            output_snippet: snippet,
        });
    }

    results
}
