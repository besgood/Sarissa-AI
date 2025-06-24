use crate::ai;
use log::info;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Finding {
    pub id: String,
    pub description: String,
    pub location: String,
    pub raw_data: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ValidationResult {
    pub id: String,
    pub is_exploitable: bool,
    pub rationale: String,
}

pub fn validate_findings(findings: &[Finding]) -> Vec<ValidationResult> {
    findings.iter().map(|f| {
        let prompt = format!(
            "Determine if this vulnerability is realistically exploitable:
            Description: {}
Location: {}
Raw Data: {}
            Respond with 'Yes' or 'No' and explain.",
            f.description, f.location, f.raw_data
        );

        let response = ai::query_llm("Vuln Exploitability Validator", &prompt);
        let is_exploitable = response.to_lowercase().contains("yes");

        info!("[Validation] {} => {}", f.id, is_exploitable);

        ValidationResult {
            id: f.id.clone(),
            is_exploitable,
            rationale: response,
        }
    }).collect()
}