
use std::fs::OpenOptions;
use std::io::Write;
use chrono::Utc;

pub fn log_event(event: &str) {
    if let Ok(mut file) = OpenOptions::new()
        .create(true)
        .append(true)
        .open("sarissa_telemetry.log")
    {
        let timestamp = Utc::now().to_rfc3339();
        let _ = writeln!(file, "[{}] {}", timestamp, event);
    }
}
