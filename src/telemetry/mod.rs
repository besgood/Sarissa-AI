// src/telemetry/mod.rs
pub mod trace;
pub mod log_hook;

use crate::ai::LLMBackend;
use crate::agents::base::AgentMemory;
use crate::react::ReasoningStep;
use crate::config::CONFIG;
use chrono::Utc;
use log::info;
use serde::{Serialize, Deserialize};
use std::fs::{create_dir_all, OpenOptions};
use std::io::Write;

#[derive(Serialize, Deserialize)]
pub struct TelemetryEvent {
    pub timestamp: String,
    pub module: String,
    pub message: String,
    pub context: Option<String>,
    pub memory: Option<AgentMemory>,
}

pub fn log_event(module: &str, message: &str, context: Option<&str>, memory: Option<&AgentMemory>) {
    let now = Utc::now().to_rfc3339();
    let event = TelemetryEvent {
        timestamp: now,
        module: module.to_string(),
        message: message.to_string(),
        context: context.map(|c| c.to_string()),
        memory: memory.cloned(),
    };

    let json = serde_json::to_string(&event).unwrap();

    if let Some(path) = &CONFIG.telemetry_log_path {
        create_dir_all("logs").unwrap();
        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(path)
            .expect("Failed to open telemetry log file");
        writeln!(file, "{}", json).expect("Failed to write to telemetry log file");
    }

    info!("[Telemetry] {}", json);
}
