//! Uses recon and fingerprints to suggest tools

use std::collections::{HashMap, HashSet};

pub fn suggest_tools(recon: &HashMap<String, String>, techs: &HashSet<String>) -> Vec<String> {
    let mut tools = Vec::new();

    if techs.contains("WordPress") {
        tools.push("wpscan".to_string());
    }
    if techs.contains("Drupal") {
        tools.push("droopescan".to_string());
    }
    if techs.contains("Flask") {
        tools.push("sqlmap".to_string());
    }

    for (k, v) in recon {
        if k.contains("Header") && v.to_lowercase().contains("cloudflare") {
            tools.push("cf-bypass-check".to_string());
        }
    }

    if tools.is_empty() {
        tools.push("nuclei".to_string());
        tools.push("ffuf".to_string());
    }

    tools
}
