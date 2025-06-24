
use std::collections::HashMap;
use crate::agents::base::{Agent, AgentMemory};

pub struct SessionAgent;

impl Agent for SessionAgent {
    fn name(&self) -> &str {
        "SessionAgent"
    }

    fn run(&self, _context: &str, memory: &AgentMemory) -> String {
        let mut session_summary = String::from("Session Summary:\n");

        for (key, value) in memory {
            if key.contains("session") || key.contains("cookie") || key.contains("auth") {
                session_summary.push_str(&format!("- {}: {}\n", key, value));
            }
        }

        if session_summary.trim() == "Session Summary:" {
            session_summary.push_str("No session-related data found.");
        }

        session_summary
    }
}
