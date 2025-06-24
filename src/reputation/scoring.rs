// src/reputation/scoring.rs

use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct ReputationScore {
    pub target: String,
    pub score: u32,
    pub tags: Vec<String>,
}

pub fn calculate_reputation(target: &str, scan_data: &str) -> ReputationScore {
    let mut score = 0;
    let mut tags = Vec::new();
    let data = scan_data.to_lowercase();

    // Simple keyword-based scoring logic
    if data.contains("wordpress") {
        score += 20;
        tags.push("wordpress".into());
    }

    if data.contains("cve") {
        score += 30;
        tags.push("known_vulns".into());
    }

    if data.contains("sql") {
        score += 25;
        tags.push("injection_possible".into());
    }

    if data.contains("admin") {
        score += 15;
        tags.push("admin_panel".into());
    }

    if data.contains("port 21") || data.contains("ftp") {
        score += 10;
        tags.push("ftp_detected".into());
    }

    if score == 0 {
        tags.push("low_risk".into());
    }

    ReputationScore {
        target: target.to_string(),
        score,
        tags,
    }
}
