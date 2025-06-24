use super::validation::ValidationResult;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct PrioritizedFinding {
    pub id: String,
    pub priority_score: u8,
    pub rationale: String,
}

pub fn prioritize_findings(results: &[ValidationResult]) -> Vec<PrioritizedFinding> {
    results.iter().map(|r| {
        let base = if r.is_exploitable { 70 } else { 30 };
        let rationale_len = r.rationale.len();
        let bonus = (rationale_len / 100).min(30) as u8;

        PrioritizedFinding {
            id: r.id.clone(),
            priority_score: base + bonus,
            rationale: r.rationale.clone(),
        }
    }).collect()
}