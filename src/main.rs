mod scanner;
mod report;
mod ai;
mod utils;
mod nessus_parser;
mod exploit;
mod analysis;
mod proxy;
mod intel;
mod fuzzer;
mod agents;

use std::env;
use std::fs;
use std::io::{self, Write};
use std::process::Command;

use crate::utils::*;
use log::{info, error};
use simple_logger;
use fuzzer::strategy::PayloadCategory;

use agents::coordinator::TaskCoordinator;
use agents::memory::AgentMemory;
use agents::recon_agent::perform_recon;
use agents::fuzz_agent::perform_fuzzing;
use agents::exploit_agent::run_exploits;
use agents::report_agent::generate_agent_summary;

fn run_suggestion(response: &str, wordlist_path: &str) {
    let lower = response.to_lowercase();

    if lower.contains("gobuster") || lower.contains("dirb") {
        info!("Running gobuster with wordlist: {}", wordlist_path);
        let gobuster_output = Command::new("gobuster")
            .args(["dir", "-u", "http://target.com", "-w", wordlist_path])
            .output();

        match gobuster_output {
            Ok(output) => {
                println!("[Gobuster Output]\n{}", clean_output(&String::from_utf8_lossy(&output.stdout)));
            }
            Err(e) => {
                error!("Failed to run gobuster: {}", e);
            }
        }
    }

    if lower.contains("nikto") {
        info!("Running Nikto on http://target.com");
        let nikto_output = Command::new("nikto")
            .args(["-h", "http://target.com"])
            .output();

        match nikto_output {
            Ok(output) => {
                println!("[Nikto Output]\n{}", clean_output(&String::from_utf8_lossy(&output.stdout)));
            }
            Err(e) => {
                error!("Failed to run Nikto: {}", e);
            }
        }
    }

    if lower.contains("sqlmap") {
        info!("Running SQLMap on http://target.com/test?id=1");
        let sqlmap_output = Command::new("sqlmap")
            .args(["-u", "http://target.com/test?id=1", "--batch"])
            .output();

        match sqlmap_output {
            Ok(output) => {
                println!("[SQLMap Output]\n{}", clean_output(&String::from_utf8_lossy(&output.stdout)));
            }
            Err(e) => {
                error!("Failed to run SQLMap: {}", e);
            }
        }
    }
}

fn main() {
    simple_logger::init_with_level(log::Level::Info).unwrap();
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        println!("Usage: sarissa <target_file> <wordlist_path> [--no-ai] [--ports <nmap_style_ports>] [--burp <burp_log.txt>] [--fuzz <url>] [--category xss|sqli|lfi|ssrf|path]");
        return;
    }

    let target_file = &args[1];
    let wordlist_path = &args[2];
    let no_ai = args.contains(&"--no-ai".to_string());

    let mut ports = "--top-ports 1000".to_string();
    let mut burp_path = None;
    let mut fuzz_url = None;
    let mut selected_category = PayloadCategory::All;

    for i in 3..args.len() {
        match args[i].as_str() {
            "--ports" if i + 1 < args.len() => ports = format!("-p {}", args[i + 1]),
            "--top-ports" if i + 1 < args.len() => ports = format!("--top-ports {}", args[i + 1]),
            "--burp" if i + 1 < args.len() => burp_path = Some(args[i + 1].clone()),
            "--fuzz" if i + 1 < args.len() => fuzz_url = Some(args[i + 1].clone()),
            "--category" if i + 1 < args.len() => {
                selected_category = match args[i + 1].to_lowercase().as_str() {
                    "xss" => PayloadCategory::XSS,
                    "sqli" => PayloadCategory::SQLi,
                    "lfi" => PayloadCategory::LFI,
                    "ssrf" => PayloadCategory::SSRF,
                    "path" => PayloadCategory::PathTraversal,
                    _ => PayloadCategory::All,
                }
            }
            _ => {}
        }
    }

    if let Some(burp_file) = burp_path {
        info!("Analyzing Burp log: {}", burp_file);
        proxy::analyze_burp_log(&burp_file, None);
        return;
    }

    if let Some(fuzz_target) = fuzz_url {
        println!("[Fuzzing] category = {:?}", selected_category);
        let payloads = fuzzer::strategy::get_payloads(selected_category);
        let mutations = fuzzer::mutation::generate_mutations(&payloads.iter().map(String::as_str).collect::<Vec<_>>());
        fuzzer::engine::start_fuzzing(&fuzz_target);
        return;
    }

    let findings = nessus_parser::parse_nessus_csv("nessus.csv");
    let _summary = nessus_parser::summarize_findings(&findings);

    let targets = fs::read_to_string(target_file).expect("Failed to read target file");
    info!("Starting scan on targets from file: {}", target_file);

    for target in targets.lines() {
        println!("\n=== Intelligence Phase for {} ===", target);
        intel::analyze_target(target);
    }

    let scan_results = scanner::run_full_scan(targets.clone(), target_file, &ports);
    let context = scan_results.join("\n");

    let llm_response = if !no_ai {
        let prompt = "Analyze the following scan results and suggest next steps.";
        let response = ai::query_llm(prompt, &context);
        println!("\n[AI Suggestion]\n{}", response);

        run_suggestion(&response, wordlist_path);

        println!("\nWas this suggestion useful? (y/n): ");
        io::stdout().flush().unwrap();
        let mut feedback = String::new();
        io::stdin().read_line(&mut feedback).unwrap();
        let useful = feedback.trim().eq_ignore_ascii_case("y");
        ai::log_feedback(prompt, &context, &response, useful);

        response
    } else {
        String::from("[AI disabled]")
    };

    // === PHASE 14: Multi-Agent Intelligence Coordination ===
    let mut memory = AgentMemory::new();

    memory.set("domain", "example.com");
    memory.set("scan_summary", "Port 80 (HTTP), 443 (HTTPS) open. Found wordpress.");
    memory.set("ai_findings", "AI suggested running wpscan and sqlmap.");
    memory.set("exploits", "CVE-2023-1234 exploit available.");

    let recon_info = perform_recon(&mut memory);
    let fuzz_info = perform_fuzzing(&mut memory);
    let exploit_result = run_exploits(&mut memory);
    let summary = generate_agent_summary(&memory);

    println!("\n[Multi-Agent Intelligence Output]");
    println!("Recon:\n{}", recon_info);
    println!("Fuzzing:\n{}", fuzz_info);
    println!("Exploitation:\n{}", exploit_result);
    println!("{}", summary);

    report::generate(&scan_results, &llm_response);
    info!("Report generation complete.");
}
