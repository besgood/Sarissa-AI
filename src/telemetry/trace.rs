use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct TraceEvent {
    pub agent: String,
    pub step: String,
    pub detail: String,
    pub timestamp: String,
}
