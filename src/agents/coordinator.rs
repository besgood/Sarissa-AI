use std::collections::HashMap;
use super::base::{Agent, AgentMemory};
use super::{recon::ReconAgent, exploit::ExploitAgent, report::ReportAgent};
use super::task_bus::{TaskBus, Task};

pub struct TaskCoordinator {
    agents: Vec<Box<dyn Agent>>,
    memory: AgentMemory,
    pub task_bus: TaskBus,
}

impl TaskCoordinator {
    pub fn new() -> Self {
        TaskCoordinator {
            agents: vec![
                Box::new(ReconAgent),
                Box::new(ExploitAgent),
                Box::new(ReportAgent),
            ],
            memory: HashMap::new(),
            task_bus: TaskBus::new(),
        }
    }

    pub fn set_memory(&mut self, key: &str, value: &str) {
        self.memory.insert(key.to_string(), value.to_string());
    }

    pub fn run_agents(&self, context: &str) -> Vec<String> {
        self.agents
            .iter()
            .map(|agent| agent.run(context, &self.memory))
            .collect()
    }

    pub fn run_and_summarize(&self, context: &str) -> String {
        let mut combined = String::new();
        for output in self.run_agents(context) {
            combined.push_str(&format!("{}\n\n", output));
        }
        combined
    }

    pub fn queue_agent_tasks(&mut self) {
        if let Some(summary) = self.memory.get("scan_summary") {
            if summary.contains("WordPress") {
                self.task_bus.add_task(Task {
                    name: "ExploitAgent".into(),
                    context: "Run WPScan".into(),
                });
            }
            if summary.contains("SQL") {
                self.task_bus.add_task(Task {
                    name: "ExploitAgent".into(),
                    context: "Run sqlmap".into(),
                });
            }
        }
    }
}
