use super::memory::AgentMemory;

pub fn perform_recon(memory: &mut AgentMemory) -> String {
    let domain = memory.get("domain").unwrap_or("unknown.com".to_string());
    let recon_result = format!("Subdomain enumeration for {}: found blog.{}, admin.{}", domain, domain, domain);

    memory.set("recon_data", &recon_result);
    recon_result
}
