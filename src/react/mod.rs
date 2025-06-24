pub mod logger;
pub mod streamer;

use crate::ai::LLMBackend;
use logger::{ReActLog, ThoughtStep};
use streamer::stream_response;

/// Entry point to trigger ReAct-style thought/action/observation logging.
pub fn run_react_analysis(prompt: &str, context: &str, backend: &dyn LLMBackend) {
    let full_prompt = format!(
        "{}\n\nContext:\n{}\n\nPlease respond using a Thought → Action → Observation pattern.",
        prompt, context
    );

    let mut log = ReActLog::new();
    stream_response(&full_prompt, backend, &mut log);

    if let Err(e) = log.save("react_log.jsonl") {
        eprintln!("Failed to save ReAct log: {}", e);
    } else {
        println!("ReAct log saved to react_log.jsonl");
    }
}


#[derive(Debug, Clone)]
pub struct ReasoningStep {
    pub thought: String,
    pub action: String,
    pub observation: String,
}
