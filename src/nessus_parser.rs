use std::fs::File;
use std::io::{BufReader, Read};
use csv::ReaderBuilder;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct NessusFinding {
    #[serde(rename = "Host")]
    pub host: String,

    #[serde(rename = "Port")]
    pub port: String,

    #[serde(rename = "Name")]
    pub plugin_name: String,

    #[serde(rename = "Plugin Output")]
    pub plugin_output: String,

    #[serde(rename = "Severity")]
    pub severity: String,

    #[serde(rename = "CVE")]
    pub cve: String,

    #[serde(rename = "Plugin ID")]
    pub plugin_id: String,
}

/// Parses a Nessus CSV report into a list of structured findings
pub fn parse_nessus_csv(path: &str) -> Vec<NessusFinding> {
    let file = File::open(path).expect("Failed to open Nessus CSV file");
    let mut rdr = ReaderBuilder::new()
        .has_headers(true)
        .from_reader(BufReader::new(file));

    let mut findings = Vec::new();
    for result in rdr.deserialize() {
        match result {
            Ok(record) => findings.push(record),
            Err(e) => eprintln!("Failed to parse line: {}", e),
        }
    }
    findings
}

/// Summarizes Nessus findings for AI
pub fn summarize_findings(findings: &[NessusFinding]) -> String {
    findings
        .iter()
        .map(|f| {
            format!(
                "Host: {}, Port: {}, Severity: {}, CVE: {}, Plugin: {}, Output: {}",
                f.host, f.port, f.severity, f.cve, f.plugin_name, f.plugin_output
            )
        })
        .collect::<Vec<_>>()
        .join("\n")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_sample() {
        let results = parse_nessus_csv("sample_nessus.csv");
        assert!(!results.is_empty());
    }
}
