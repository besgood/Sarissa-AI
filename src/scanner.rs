use crate::scanner_tools::{ScanTool, FFUFScan, AmassScan, HttpxScan, NucleiScan};
use log::info;

/// Run all configured scanners and aggregate results
pub fn run_full_scan(targets: String, target_path: &str, ports: &str) -> Vec<String> {
    let mut results = Vec::new();

    // Derive test values from targets file for demo purposes
    let lines: Vec<&str> = targets.lines().collect();
    let first_target = lines.get(0).unwrap_or(&"http://example.com").trim().to_string();

    // Hardcoded setup for demonstration (in real use, pass these via args or AI)
    let tools: Vec<Box<dyn ScanTool>> = vec![
        Box::new(FFUFScan {
            url: first_target.clone(),
            wordlist: "/usr/share/wordlists/dirb/common.txt".to_string(),
        }),
        Box::new(AmassScan {
            domain: first_target.replace("http://", "").replace("https://", ""),
        }),
        Box::new(HttpxScan {
            urls: vec![first_target.clone()],
        }),
        Box::new(NucleiScan {
            urls: vec![first_target.clone()],
        }),
    ];

    for tool in tools {
        info!("Running tool: {}", tool.name());
        let output = tool.run();
        results.extend(output);
    }

    results
}

/// Build scanner tools based on AI suggestion
pub fn build_scan_tools_from_ai(ai_response: &str, target: &str, wordlist: &str) -> Vec<Box<dyn ScanTool>> {
    let mut tools: Vec<Box<dyn ScanTool>> = Vec::new();
    let lower = ai_response.to_lowercase();

    if lower.contains("ffuf") {
        tools.push(Box::new(FFUFScan {
            url: target.to_string(),
            wordlist: wordlist.to_string(),
        }));
    }

    if lower.contains("amass") {
        tools.push(Box::new(AmassScan {
            domain: target.replace("http://", "").replace("https://", ""),
        }));
    }

    if lower.contains("httpx") {
        tools.push(Box::new(HttpxScan {
            urls: vec![target.to_string()],
        }));
    }

    if lower.contains("nuclei") {
        tools.push(Box::new(NucleiScan {
            urls: vec![target.to_string()],
        }));
    }

    tools
}
