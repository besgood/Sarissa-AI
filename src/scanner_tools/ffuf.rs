use std::process::Command;
use crate::utils::clean_output;
use log::{info, error};

/// Run ffuf against a target URL with a given wordlist
pub fn run_ffuf(target_url: &str, wordlist_path: &str) -> Vec<String> {
    let mut results = Vec::new();

    info!("Running ffuf on {} with wordlist {}", target_url, wordlist_path);

    let output = Command::new("ffuf")
        .args(&["-u", &format!("{}/FUZZ", target_url), "-w", wordlist_path, "-mc", "200,204,301,302,307,403"])
        .output();

    match output {
        Ok(output) => {
            let cleaned = clean_output(&String::from_utf8_lossy(&output.stdout));
            if cleaned.contains(":: Progress ::") {
                for line in cleaned.lines() {
                    if line.contains("[Status:") {
                        results.push(line.to_string());
                    }
                }
            } else {
                results.push(cleaned);
            }
        }
        Err(e) => {
            error!("Failed to run ffuf: {}", e);
            results.push(format!("[ERROR] ffuf execution failed: {}", e));
        }
    }

    results
}
