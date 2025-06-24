use std::fs::{OpenOptions};
use std::io::Write;
use chrono::Utc;
use serde::{Serialize};
use std::sync::Mutex;
use lazy_static::lazy_static;

#[derive(Serialize, Debug)]
pub struct TraceEvent {
    pub timestamp: String,
    pub component: String,
    pub message: String,
}

lazy_static! {
    static ref LOG_FILE: Mutex<std::fs::File> = {
        let file = OpenOptions::new()
            .create(true)
            .append(true)
            .open("logs/trace.json")
            .expect("Unable to open trace log file");
        Mutex::new(file)
    };
}

pub fn log_event(component: &str, message: &str) {
    let event = TraceEvent {
        timestamp: Utc::now().to_rfc3339(),
        component: component.to_string(),
        message: message.to_string(),
    };

    if let Ok(json) = serde_json::to_string(&event) {
        if let Ok(mut file) = LOG_FILE.lock() {
            let _ = writeln!(file, "{}", json);
        }
    }
}
