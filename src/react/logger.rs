use serde::{Serialize, Deserialize};
use std::fs::File;
use std::io::{BufWriter, Write};

#[derive(Debug, Serialize, Deserialize)]
pub struct ThoughtStep {
    pub step: usize,
    pub thought: String,
    pub action: String,
    pub observation: String,
}

#[derive(Debug)]
pub struct ReActLog {
    pub steps: Vec<ThoughtStep>,
}

impl ReActLog {
    pub fn new() -> Self {
        Self { steps: vec![] }
    }

    pub fn add_step(&mut self, thought: &str, action: &str, observation: &str) {
        let step = ThoughtStep {
            step: self.steps.len() + 1,
            thought: thought.to_string(),
            action: action.to_string(),
            observation: observation.to_string(),
        };
        self.steps.push(step);
    }

    pub fn save(&self, path: &str) -> std::io::Result<()> {
        let file = File::create(path)?;
        let mut writer = BufWriter::new(file);

        for step in &self.steps {
            let json_line = serde_json::to_string(step)?;
            writeln!(writer, "{}", json_line)?;
        }

        Ok(())
    }
}
