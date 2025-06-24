//! Classifies retested findings into basic risk categories

use crate::analysis::retesting::RetestResult;

/// Output structure for a fully enriched analysis
#[derive(Debug, Clone)]
pub struct RiskFinding {
    pub tool: String,
    pub description: String,
    pub severity: String,
    pub snippet: String,
}

/// Simple heuristic risk model
pub fn classify_risks(results: &[RetestResult]) -> Vec<RiskFinding> {
    let mut risks = Vec::new();

    for r in results {
        let sev = if r.success {
            if r.finding.suggested_tool == "sqlmap" {
                "high"
            } else if r.finding.suggested_tool == "ffuf" {
                "medium"
            } else {
                "low"
            }
        } else {
            "info"
        };

        risks.push(RiskFinding {
            tool: r.finding.suggested_tool.clone(),
            description: r.finding.reason.clone(),
            severity: sev.to_string(),
            snippet: r.output_snippet.clone(),
        });
    }

    risks
}
