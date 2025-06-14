use std::collections::HashMap;

/// Shared interface for all agents
pub trait Agent {
    fn name(&self) -> &str;
    fn run(&self, context: &str, memory: &HashMap<String, String>) -> String;
}

/// Shared memory structure used by agents
pub type AgentMemory = HashMap<String, String>;
