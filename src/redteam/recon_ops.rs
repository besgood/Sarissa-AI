pub fn internal_recon() -> Vec<String> {
    let mut output = Vec::new();
    output.push("whoami".to_string());
    output.push("hostname".to_string());
    output.push("ip a".to_string());
    output.push("netstat -tulnp".to_string());
    output.push("ps aux".to_string());
    output
}
