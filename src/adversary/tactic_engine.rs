use crate::ai::query_llm;
use crate::utils::clean_output;
use crate::agents::coordinator::TaskCoordinator;
use log::info;

/// Generates adversarial red team tactics from AI based on context.
pub fn generate_red_team_tactics(context: &str, coordinator: &TaskCoordinator) -> String {
    let prompt = r#"
You are a red team operator. Based on the following findings, suggest your next adversarial steps.
Consider advanced tactics like WAF bypass, SSRF, SSTI, chained exploitation, evasion, lateral movement.
Context:
"#;

    let memory_dump = coordinator.debug_memory();
    let combined_context = format!("{prompt}\n{}\n\nMemory:\n{}", context, memory_dump);
    let response = query_llm("red-team-tactics", &combined_context);

    info!("AI-generated adversarial tactics:\n{}", response);
    clean_output(&response)
}
