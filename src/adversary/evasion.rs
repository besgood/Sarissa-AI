use crate::ai::query_llm;
use crate::intel::payload_memory::{PayloadRecord, PayloadHistory};
use log::{info, warn};

/// Suggests AI-driven WAF evasion payloads based on failed inputs.
pub fn generate_evasion_techniques(history: &PayloadHistory, category: &str) -> Vec<String> {
    let recent_failed: Vec<&PayloadRecord> = history
        .records
        .iter()
        .rev()
        .filter(|r| r.category == category && !r.successful)
        .take(10)
        .collect();

    if recent_failed.is_empty() {
        warn!("No failed payloads found for evasion generation.");
        return vec![];
    }

    let context = recent_failed
        .iter()
        .map(|r| format!("- {}", r.payload))
        .collect::<Vec<_>>()
        .join("\n");

    let prompt = format!(
        "These payloads were blocked by a WAF. Generate 5 modified variants to attempt bypass:\n{}",
        context
    );

    let response = query_llm("waf-bypass", &prompt);
    response.lines().map(|s| s.trim().to_string()).collect()
}
