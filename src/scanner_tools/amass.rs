use std::process::Command;
use crate::utils::clean_output;
use log::{info, error};

/// Run amass in passive mode against a given domain
pub fn run_amass(domain: &str) -> Vec<String> {
    let mut results = Vec::new();

    info!("Running amass in passive mode for domain: {}", domain);

    let output = Command::new("amass")
        .args(["enum", "-passive", "-d", domain])
        .output();

    match output {
        Ok(output) => {
            let cleaned = clean_output(&String::from_utf8_lossy(&output.stdout));
            for line in cleaned.lines() {
                if !line.trim().is_empty() {
                    results.push(line.to_string());
                }
            }
        }
        Err(e) => {
            error!("Failed to run amass: {}", e);
            results.push(format!("[ERROR] amass execution failed: {}", e));
        }
    }

    results
}
