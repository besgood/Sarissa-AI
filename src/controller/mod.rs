
use crate::observer::log_thought_action_observation;

pub fn perform_action_with_observation(thought: &str, action: &str) {
    let observation = simulate_observation(action);
    log_thought_action_observation(thought, action, &observation);
}

fn simulate_observation(action: &str) -> String {
    format!("Simulated observation of '{}'", action)
}
