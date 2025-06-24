mod social_engineering;
mod telemetry;
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
mod react;
mod config;
mod reputation;

use std::env;
use std::fs;
use std::io::{self, Write};
use std::process::Command;
use log::{info, error};
use simple_logger;
use simulation::{profile::TargetProfile, planner::generate_ai_attack_plan};


use crate::utils::*;
use crate::redteam::c2::C2Session;
use crate::redteam::post_exploitation::suggest_post_exploitation;
use redteam::sliver::launch_sliver_listener;
use redteam::mythic::start_mythic_services;
use redteam::cobalt::launch_cobalt_strike_teamserver;

use fuzzer::strategy::PayloadCategory;
use agents::coordinator::TaskCoordinator;
use proxy::proxy_chain::run_proxy_chain;
use proxy::session_store::SessionStore;

fn run_suggestion(response: &str, wordlist_path: &str) {
    let lower = response.to_lowercase();

    if lower.contains("gobuster") {
        info!("Running gobuster with wordlist: {}", wordlist_path);
        let output = Command::new("gobuster")
            .args(["dir", "-u", "http://target.com", "-w", wordlist_path])
            .output();
        match output {
            Ok(o) => println!("[Gobuster]\n{}", clean_output(&String::from_utf8_lossy(&o.stdout))),
            Err(e) => error!("Gobuster failed: {}", e),
        }
    }

    if lower.contains("nikto") {
        info!("Running Nikto on http://target.com");
        let output = Command::new("nikto")
            .args(["-h", "http://target.com"])
            .output();
        match output {
            Ok(o) => println!("[Nikto]\n{}", clean_output(&String::from_utf8_lossy(&o.stdout))),
            Err(e) => error!("Nikto failed: {}", e),
        }
    }

    if lower.contains("sqlmap") {
        info!("Running SQLMap on http://target.com/test?id=1");
        let output = Command::new("sqlmap")
            .args(["-u", "http://target.com/test?id=1", "--batch"])
            .output();
        match output {
            Ok(o) => println!("[SQLMap]\n{}", clean_output(&String::from_utf8_lossy(&o.stdout))),
            Err(e) => error!("SQLMap failed: {}", e),
        }
    }
}

fn main() {
    telemetry::logger::log_trace_event(
        "ReconAgent",
        "Start Scan",
        "Initialized scan for 192.168.1.1"
    );
    let pretext = social_engineering::generate_phishing_pretext("Alice");
    social_engineering::save_pretext(&pretext);
    telemetry::log_event(
        "ReconAgent",
        "Scanning for open ports",
        "Running rustscan",
        "Found ports 80 and 443 open"
    );
    simple_logger::init_with_level(log::Level::Info).unwrap();
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        println!("Usage: sarissa <target_file> <wordlist> [--sliver] [--mythic] [--cobalt <ip> <password>] [--no-ai] [--ports <range>] [--burp <file>] [--fuzz <url>] [--category xss|sqli|lfi|ssrf|path] [--proxy-chain <chain.json>]");
        return;
    }

    
    if args.contains(&"--sliver".to_string()) {
        launch_sliver_listener();
        return;
    }

    if args.contains(&"--mythic".to_string()) {
        start_mythic_services();
        return;
    }

    if let Some(pos) = args.iter().position(|x| x == "--cobalt") {
        if pos + 2 < args.len() {
            let ip = &args[pos + 1];
            let password = &args[pos + 2];
            launch_cobalt_strike_teamserver(ip, password);
            return;
        } else {
            eprintln!("Usage: --cobalt <ip> <password>");
            return;
        }
    }

let target_file = &args[1];
    let wordlist_path = &args[2];
    let no_ai = args.contains(&"--no-ai".to_string());

    let mut ports = "--top-ports 1000".to_string();
    let mut burp_path = None;
    let mut fuzz_url = None;
    let mut proxy_chain_path = None;
    let mut selected_category = PayloadCategory::All;

    for i in 3..args.len() {
        match args[i].as_str() {
            "--ports" if i + 1 < args.len() => ports = format!("-p {}", args[i + 1]),
            "--top-ports" if i + 1 < args.len() => ports = format!("--top-ports {}", args[i + 1]),
            "--burp" if i + 1 < args.len() => burp_path = Some(args[i + 1].clone()),
            "--fuzz" if i + 1 < args.len() => fuzz_url = Some(args[i + 1].clone()),
            "--proxy-chain" if i + 1 < args.len() => proxy_chain_path = Some(args[i + 1].clone()),
            "--category" if i + 1 < args.len() => {
                selected_category = match args[i + 1].to_lowercase().as_str() {
                    "xss" => PayloadCategory::XSS,
                    "sqli" => PayloadCategory::SQLi,
                    "lfi" => PayloadCategory::LFI,
                    "ssrf" => PayloadCategory::SSRF,
                    "path" => PayloadCategory::PathTraversal,
                    _ => PayloadCategory::All,
                };
            }
            _ => {}
        }
    }

    if let Some(chain_path) = proxy_chain_path {
        let mut session_store = SessionStore::new();
        run_proxy_chain(&chain_path, &mut session_store);
        return;
    }

    if let Some(burp_file) = burp_path {
        info!("Analyzing Burp log: {}", burp_file);
        proxy::analyze_burp_log(&burp_file, None);
        return;
    }

    if let Some(fuzz_target) = fuzz_url {
        println!("[Fuzzing] category = {:?}", selected_category);
        let payloads = fuzzer::strategy::get_payloads(selected_category);
        let _mutations = fuzzer::mutation::generate_mutations(&payloads.iter().map(String::as_str).collect::<Vec<_>>());
        fuzzer::engine::start_fuzzing(&fuzz_target, selected_category);
        return;
    }

    let findings = nessus_parser::parse_nessus_csv("nessus.csv");
    let summary = nessus_parser::summarize_findings(&findings);

    let targets = fs::read_to_string(target_file).expect("Failed to read target file");
    info!("Scanning targets from: {}", target_file);

    for target in targets.lines() {
        println!("\n=== Recon Phase for {} ===", target);
        intel::analyze_target(target);
    }

    let scan_results = scanner::run_full_scan(targets, target_file, &ports);
    let context = scan_results.join("\n");

    let llm_response = if !no_ai {
        let prompt = "Analyze scan results and suggest the next best actions.";
        let response = ai::query_llm(prompt, &context);
        println!("\n[AI Suggestion]\n{}", response);
        run_suggestion(&response, wordlist_path);

        print!("Was this useful? (y/n): ");
        io::stdout().flush().unwrap();
        let mut feedback = String::new();
        io::stdin().read_line(&mut feedback).unwrap();
        let useful = feedback.trim().eq_ignore_ascii_case("y");
        ai::log_feedback(prompt, &context, &response, useful);
        response
    } else {
        String::from("[AI disabled]")
    };

    let enriched = analysis::run_analysis(&scan_results);
    for e in &enriched {
        println!("[Enriched] {}", e);
    }

    let mut coordinator = TaskCoordinator::new();
    coordinator.set_memory("domain", "example.com");
    coordinator.set_memory("scan_summary", &summary);
    coordinator.set_memory("ai_findings", &llm_response);
    coordinator.set_memory("exploits", "Example CVE-2024-0001");

    let agent_summary = coordinator.run_and_summarize(&context);
    println!("\n[Agent Summary]\n{}", agent_summary);

    react::run_react_analysis(
        "Analyze findings and perform ReAct-style reasoning.",
        &format!("{} | Summary: {}", context, summary),
        &*ai::get_backend(),
    );
	
	// Phase 18 – Simulated Target Modeling
let mut profile = TargetProfile::new("example.com");
profile.set_os("Linux");
profile.add_port(80, "http");
profile.add_port(443, "https");
profile.add_web_tech("WordPress");

let playbook = generate_ai_attack_plan(&profile);
println!("\n[Simulated Attack Plan]\n{}", playbook);


let session = C2Session::new("sess01", "192.168.0.10", "bob", "user");
session.log();

let tasks = suggest_post_exploitation(&session);
for task in tasks {
    println!("[AI Post-Exploitation Suggestion] {}", task);
}


    report::generate(&scan_results, &llm_response);
    info!("Report complete.");
}
