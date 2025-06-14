use std::process::Command;
use std::fs::File;
use std::io::Write;
use tempfile::NamedTempFile;
use crate::utils::clean_output;
use log::{info, error};

/// Run nuclei on a list of URLs using default templates
pub fn run_nuclei(urls: &[String]) -> Vec<String> {
    let mut results = Vec::new();

    // Create a temporary file to hold the list of URLs
    let mut temp_file = match NamedTempFile::new() {
        Ok(f) => f,
        Err(e) => {
            error!("Failed to create temp file for nuclei: {}", e);
            return vec![format!("[ERROR] nuclei temp file error: {}", e)];
        }
    };

    for url in urls {
        writeln!(temp_file, "{}", url).unwrap();
    }

    let path = temp_file.path().to_str().unwrap();
    info!("Running nuclei on URLs listed in: {}", path);

    let output = Command::new("nuclei")
        .args(["-l", path, "-silent"])
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
            error!("Failed to run nuclei: {}", e);
            results.push(format!("[ERROR] nuclei execution failed: {}", e));
        }
    }

    results
}
