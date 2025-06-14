//! Maps scan findings to suggested tool actions

/// Placeholder structure for a mapped finding
#[derive(Debug, Clone)]
pub struct CorrelatedFinding {
    pub raw: String,
    pub suggested_tool: String,
    pub reason: String,
}

/// Maps raw scan results to tools (simple matching rules for now)
pub fn map_findings(scan_results: &[String]) -> Vec<CorrelatedFinding> {
    let mut mapped = Vec::new();

    for line in scan_results {
        let mut tool = "nuclei".to_string();
        let mut reason = "default mapping".to_string();

        if line.contains("login") {
            tool = "ffuf".to_string();
            reason = "Login interface detected — test for bruteforce or bypass".to_string();
        } else if line.contains("php") {
            tool = "nikto".to_string();
            reason = "PHP backend — Nikto can find outdated scripts".to_string();
        } else if line.contains("port 3306") {
            tool = "sqlmap".to_string();
            reason = "MySQL detected — test for SQL injection".to_string();
        }

        mapped.push(CorrelatedFinding {
            raw: line.clone(),
            suggested_tool: tool,
            reason,
        });
    }

    mapped
}
