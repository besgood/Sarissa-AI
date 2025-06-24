use crate::ai::LLMBackend;
use crate::react::logger::ReActLog;

/// Very naive example that simulates a streaming response.
/// Replace this with a real call to Ollama or LLM backend that streams step-by-step.
pub fn stream_response(prompt: &str, backend: &dyn LLMBackend, log: &mut ReActLog) {
    println!("[ReAct] Streaming thoughts...");

    let fake_response = vec![
        ("Let’s identify web tech used.", "Run WhatWeb", "Found WordPress 5.8"),
        ("Check for known WP vulns.", "Run WPScan", "WPScan found CVE-2022-12345"),
        ("Test SQLi on login endpoint", "Run sqlmap", "sqlmap found injectable param: username"),
    ];

    for (thought, action, observation) in fake_response {
        println!("→ Thought: {}\n  Action: {}\n  Observation: {}", thought, action, observation);
        log.add_step(thought, action, observation);
    }
}
