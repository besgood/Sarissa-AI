use std::fs::OpenOptions;
use std::io::Write;
use chrono::Local;
use std::env;

use crate::ai_reasoning::memory;
use crate::ai_reasoning::prompts;

/// Queries the local LLM or remote API
pub fn query_llm(prompt: &str, context: &str) -> String {
    let full_prompt = format!("{}\n\nContext:\n{}", prompt, context);

    // Placeholder logic — integrate with actual model or API
    let simulated_response = "Run ffuf and nuclei based on open HTTP port and headers.";
    simulated_response.to_string()
}

/// Logs the AI suggestion, context, and user approval
pub fn log_feedback(prompt: &str, context: &str, response: &str, useful: bool) {
    let timestamp = Local::now();
    let filename = "ai_feedback_log.jsonl";

    let record = format!(
        "{{\"timestamp\":\"{}\",\"prompt\":\"{}\",\"context\":\"{}\",\"response\":\"{}\",\"useful\":{}}}\n",
        timestamp.format("%Y-%m-%dT%H:%M:%S"),
        prompt.replace('"', "'"),
        context.replace('"', "'"),
        response.replace('"', "'"),
        useful
    );

    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(filename)
        .expect("Unable to open feedback log file");
    file.write_all(record.as_bytes()).unwrap();

    if useful {
        save_ai_memory(context, response);
    }
}

/// Appends a successful AI recommendation to ai_memory_log.txt
pub fn save_ai_memory(context: &str, response: &str) {
    let timestamp = Local::now();
    let entry = format!(
        "---\n[{}]\nContext:\n{}\nAI Response:\n{}\n",
        timestamp.format("%Y-%m-%d %H:%M:%S"),
        context.trim(),
        response.trim()
    );

    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("ai_memory_log.txt")
        .expect("Unable to write memory log");
    file.write_all(entry.as_bytes()).unwrap();
}

/// Builds context-aware prompt using past memory and new scan data
pub fn generate_contextual_prompt(scan_data: &str) -> String {
    let memory_context = memory::build_memory_context("ai_memory_log.txt", 5);
    format!("{}\n\n{}", memory_context, scan_data)
}


#[derive(Debug, Clone)]
pub enum LLMBackend {
    LLaMA3Local,
    RemoteAPI(String),
}

pub fn get_backend() -> Box<dyn std::fmt::Debug> {
    // In production this could switch based on config
    Box::new(LLMBackend::LLaMA3Local)
}
