use crate::ai::query_llm;

pub fn select_tactic(context: &str) -> String {
    let prompt = format!(
        "Given the red team context below, suggest next MITRE ATT&CK tactic:\n{}",
        context
    );
    query_llm(&prompt, "")
}
