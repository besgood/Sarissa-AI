use std::collections::HashMap;
use super::base::{Agent, AgentMemory};

pub struct ReconAgent;

impl Agent for ReconAgent {
    fn name(&self) -> &str {
        "ReconAgent"
    }

    fn run(&self, context: &str, memory: &AgentMemory) -> String {
        let mut output = String::new();
        output.push_str("[ReconAgent] Beginning reconnaissance analysis...\n");

        if context.contains("open ports") {
            output.push_str("- Detected open ports, recommend running `httpx` and `amass`\n");
        }

        if let Some(domain) = memory.get("domain") {
            output.push_str(&format!("- Target domain detected: {}\n", domain));
            output.push_str("- Suggest subdomain enumeration via `amass` or `dnsx`\n");
        }

        output.push_str("[ReconAgent] Reconnaissance phase complete.");
        output
    }
}
