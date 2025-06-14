use std::process::Command;
use crate::utils::clean_output;
use log::{info, error};

/// Run httpx against a list of URLs (stored in a temporary file)
pub fn run_httpx(urls: &[String]) -> Vec<String> {
    use std::fs::File;
    use std::io::Write;
    use tempfile::NamedTempFile;

    let mut results = Vec::new();

    // Write URLs to a temporary file
    let mut temp_file = match NamedTempFile::new() {
        Ok(f) => f,
        Err(e) => {
            error!("Failed to create temp file for httpx: {}", e);
            return vec![format!("[ERROR] httpx temp file error: {}", e)];
        }
    };

    for url in urls {
        writeln!(temp_file, "{}", url).unwrap();
    }

    let path = temp_file.path().to_str().unwrap();
    info!("Running httpx on URLs listed in: {}", path);

    let output = Command::new("httpx")
        .args(["-silent", "-status-code", "-title", "-tech-detect", "-l", path])
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
            error!("Failed to run httpx: {}", e);
            results.push(format!("[ERROR] httpx execution failed: {}", e));
        }
    }

    results
}
