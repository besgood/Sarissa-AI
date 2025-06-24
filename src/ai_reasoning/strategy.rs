use std::collections::HashMap;

/// Given open ports or service banners, return a list of suggested tools and justifications.
pub fn analyze_targets(services: &[(String, u16, String)]) -> Vec<(String, String)> {
    let mut suggestions = Vec::new();

    for (ip, port, banner) in services {
        if *port == 80 || *port == 443 {
            suggestions.push((
                "ffuf".to_string(),
                format!("{}:{} has HTTP service → ffuf for directory brute-forcing", ip, port),
            ));
        }
        if banner.to_lowercase().contains("apache") || banner.to_lowercase().contains("nginx") {
            suggestions.push((
                "nikto".to_string(),
                format!("{}:{} banner shows {} → nikto for web server vulnerabilities", ip, port, banner),
            ));
        }
        if *port == 21 {
            suggestions.push((
                "ftp-anon".to_string(),
                format!("{}:{} is FTP → test for anonymous access", ip, port),
            ));
        }
        if *port == 445 {
            suggestions.push((
                "enum4linux".to_string(),
                format!("{}:{} is SMB → enum4linux for share/user enumeration", ip, port),
            ));
        }
    }

    suggestions
}

/// Summarize a suggestion list into a string for prompting or logging
pub fn summarize_strategies(suggestions: &[(String, String)]) -> String {
    suggestions
        .iter()
        .map(|(tool, reason)| format!("[{}] {}", tool.to_uppercase(), reason))
        .collect::<Vec<String>>()
        .join("\n")
}
