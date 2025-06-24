
use std::collections::HashMap;
use crate::proxy::burp_parser::BurpRequest;

#[derive(Debug, Default)]
pub struct FlowTracker {
    pub sessions: HashMap<String, Vec<BurpRequest>>,
}

impl FlowTracker {
    pub fn new() -> Self {
        FlowTracker {
            sessions: HashMap::new(),
        }
    }

    pub fn add_request(&mut self, session_id: &str, request: BurpRequest) {
        self.sessions
            .entry(session_id.to_string())
            .or_default()
            .push(request);
    }

    pub fn get_flow(&self, session_id: &str) -> Option<&Vec<BurpRequest>> {
        self.sessions.get(session_id)
    }

    pub fn all_flows(&self) -> &HashMap<String, Vec<BurpRequest>> {
        &self.sessions
    }
}
