//! Parses Burp Suite request logs (plain text format only)

use std::fs;
use std::io::{BufRead, BufReader};
use std::fs::File;
use std::error::Error;

#[derive(Debug, Clone)]
pub struct BurpRequest {
    pub method: String,
    pub url: String,
    pub headers: Vec<String>,
    pub body: Option<String>,
}

/// Parses a simple Burp request file (one per block)
pub fn parse_burp_file(path: &str) -> Result<Vec<BurpRequest>, Box<dyn Error>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let mut requests = Vec::new();

    let mut current_lines = Vec::new();
    for line in reader.lines() {
        let line = line?;
        if line.trim().is_empty() && !current_lines.is_empty() {
            if let Some(req) = parse_request_block(&current_lines) {
                requests.push(req);
            }
            current_lines.clear();
        } else {
            current_lines.push(line);
        }
    }
    if !current_lines.is_empty() {
        if let Some(req) = parse_request_block(&current_lines) {
            requests.push(req);
        }
    }

    Ok(requests)
}

fn parse_request_block(lines: &[String]) -> Option<BurpRequest> {
    if lines.is_empty() {
        return None;
    }
    let mut method = "GET".to_string();
    let mut url = "".to_string();
    let mut headers = Vec::new();
    let mut body_lines = Vec::new();
    let mut in_body = false;

    for line in lines {
        if line.starts_with("GET") || line.starts_with("POST") || line.starts_with("PUT") || line.starts_with("DELETE") {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() >= 2 {
                method = parts[0].to_string();
                url = parts[1].to_string();
            }
        } else if in_body {
            body_lines.push(line.clone());
        } else if line.trim().is_empty() {
            in_body = true;
        } else {
            headers.push(line.clone());
        }
    }

    Some(BurpRequest {
        method,
        url,
        headers,
        body: if body_lines.is_empty() { None } else { Some(body_lines.join("\n")) },
    })
}
