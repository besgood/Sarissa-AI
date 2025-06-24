// src/malicious_db/enricher.rs

use crate::ai::query_llm;
use crate::malicious_db::malware_lookup::MalwareSignature;

pub fn enrich_with_ai(signatures: &[MalwareSignature]) -> Vec<String> {
    let mut enriched = Vec::new();

    for sig in signatures {
        let context = format!(
            "Indicator: {}\nDescription: {}\nSeverity: {}\nFamily: {:?}",
            sig.indicator, sig.description, sig.severity, sig.family
        );

        let prompt = "Analyze this indicator for potential threats and recommend containment steps.";
        let response = query_llm(prompt, &context);
        enriched.push(response);
    }

    enriched
}
