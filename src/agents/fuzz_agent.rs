use super::memory::AgentMemory;

pub fn perform_fuzzing(memory: &mut AgentMemory) -> String {
    let target = memory.get("scan_summary").unwrap_or("no target".to_string());
    let fuzz_result = format!("Fuzzed endpoints for {}: Found reflected XSS at /search?q=", target);

    memory.set("fuzz_findings", &fuzz_result);
    fuzz_result
}
