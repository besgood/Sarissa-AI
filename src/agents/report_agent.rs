use super::memory::AgentMemory;

pub fn generate_agent_summary(memory: &AgentMemory) -> String {
    let recon = memory.get("recon_data").unwrap_or_default();
    let fuzz = memory.get("fuzz_findings").unwrap_or_default();
    let exploit = memory.get("exploit_result").unwrap_or_default();

    format!(
        "[Agent Report Summary]\n{}\n{}\n{}\n",
        recon, fuzz, exploit
    )
}
