use super::payload_memory::{PayloadHistory, PayloadRecord};
use crate::ai::query_llm;

pub fn generate_adaptive_payloads(history: &PayloadHistory, category: &str) -> Vec<String> {
    let successful = history
        .filter_by_outcome(category, true)
        .into_iter()
        .map(|r| r.payload)
        .collect::<Vec<_>>();

    let failed = history
        .filter_by_outcome(category, false)
        .into_iter()
        .map(|r| r.payload)
        .collect::<Vec<_>>();

    let prompt = format!(
        "Based on past results, generate 5 advanced payloads for category '{}'.\n\n\
        Successful payloads:\n{}\n\n\
        Failed payloads:\n{}\n\n\
        Your output should be a JSON array of new payloads.",
        category,
        successful.join("\n"),
        failed.join("\n")
    );

    let response = query_llm("Generate adaptive fuzzing payloads", &prompt);
    parse_payloads_from_json(&response)
}

fn parse_payloads_from_json(response: &str) -> Vec<String> {
    match serde_json::from_str::<Vec<String>>(response) {
        Ok(payloads) => payloads,
        Err(_) => {
            eprintln!("Failed to parse AI payloads. Raw response: {}", response);
            vec![]
        }
    }
}
