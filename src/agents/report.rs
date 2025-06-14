use std::collections::HashMap;
use super::base::{Agent, AgentMemory};

pub struct ReportAgent;

impl Agent for ReportAgent {
    fn name(&self) -> &str {
        "ReportAgent"
    }

    fn run(&self, context: &str, memory: &AgentMemory) -> String {
        let mut report = String::new();
        report.push_str("[ReportAgent] Compiling summary report...\n");

        if let Some(summary) = memory.get("scan_summary") {
            report.push_str(&format!("- Scan Summary:\n{}\n", summary));
        }

        if let Some(ai_findings) = memory.get("ai_findings") {
            report.push_str(&format!("- AI-Driven Recommendations:\n{}\n", ai_findings));
        }

        if let Some(exploits) = memory.get("exploits") {
            report.push_str(&format!("- Potential Exploits:\n{}\n", exploits));
        }

        report.push_str("[ReportAgent] Report generation complete.");
        report
    }
}
