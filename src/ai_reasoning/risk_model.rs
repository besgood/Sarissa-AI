use std::collections::HashMap;

/// Assigns a basic risk score to a given port and service banner
/// Higher score = more likely to be vulnerable or important
pub fn score_service(port: u16, banner: &str) -> u8 {
    match port {
        21 => 7,    // FTP
        22 => 4,    // SSH
        23 => 8,    // Telnet
        25 => 5,    // SMTP
        80 => 6,    // HTTP
        443 => 6,   // HTTPS
        3306 => 9,  // MySQL
        3389 => 9,  // RDP
        445 => 8,   // SMB
        _ => {
            if banner.to_lowercase().contains("apache") {
                7
            } else if banner.to_lowercase().contains("nginx") {
                6
            } else {
                3
            }
        }
    }
}

/// Scores a full target list and returns a list sorted by descending risk
pub fn rank_targets(services: &[(String, u16, String)]) -> Vec<(String, u16, String, u8)> {
    let mut scored = services
        .iter()
        .map(|(ip, port, banner)| {
            let score = score_service(*port, banner);
            (ip.clone(), *port, banner.clone(), score)
        })
        .collect::<Vec<_>>();

    scored.sort_by(|a, b| b.3.cmp(&a.3));
    scored
}