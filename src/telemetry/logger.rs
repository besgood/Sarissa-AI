use std::fs::{OpenOptions, create_dir_all};
use std::io::Write;
use std::path::Path;
use chrono::Utc;
use crate::telemetry::trace::TraceEvent;

pub fn log_trace_event(agent: &str, step: &str, detail: &str) {
    let timestamp = Utc::now().to_rfc3339();
    let event = TraceEvent {
        agent: agent.to_string(),
        step: step.to_string(),
        detail: detail.to_string(),
        timestamp,
    };

    let json = match serde_json::to_string(&event) {
        Ok(j) => j,
        Err(e) => {
            eprintln!("Failed to serialize trace event: {}", e);
            return;
        }
    };

    let path = Path::new("logs/trace.json");
    if let Some(parent) = path.parent() {
        if let Err(e) = create_dir_all(parent) {
            eprintln!("Failed to create log directory: {}", e);
            return;
        }
    }

    let mut file = match OpenOptions::new().create(true).append(true).open(path) {
        Ok(f) => f,
        Err(e) => {
            eprintln!("Failed to open trace log file: {}", e);
            return;
        }
    };

    if let Err(e) = writeln!(file, "{}", json) {
        eprintln!("Failed to write trace event: {}", e);
    }
}
