use crate::intel::payload_engine::generate_adaptive_payloads;
use crate::intel::payload_memory::{PayloadHistory, PayloadRecord};
use super::strategy::PayloadCategory;
use log::info;
use std::time::Duration;
use std::thread;

pub fn start_fuzzing(target_url: &str, category: PayloadCategory) {
    let mut history = PayloadHistory::load_or_default("payload_history.json");
    let category_str = format!("{:?}", category).to_lowercase();

    info!("Generating AI-adaptive payloads for category: {}", category_str);
    let ai_payloads = generate_adaptive_payloads(&history, &category_str);

    if ai_payloads.is_empty() {
        eprintln!("No AI payloads generated. Fallback to static strategy.");
        return;
    }

    for payload in ai_payloads {
        info!("Testing payload: {}", payload);

        // Simulate test delay and dummy result
        thread::sleep(Duration::from_millis(500));

        let successful = simulate_fuzz_request(target_url, &payload);

        history.add(PayloadRecord {
            category: category_str.clone(),
            payload,
            successful,
        });
    }

    history.save("payload_history.json");
    info!("Payload testing and history update complete.");
}

// Dummy function — replace with real HTTP request logic.
fn simulate_fuzz_request(_target_url: &str, payload: &str) -> bool {
    // This is a mock: treat payloads with "bypass" in them as successful.
    payload.to_lowercase().contains("bypass")
}
