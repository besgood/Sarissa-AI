//! Entry point for fuzzing and mutation loop

pub mod engine;
pub mod mutation;
pub mod evaluator;

use crate::fuzzer::engine;

/// Kicks off fuzzing workflow on a target endpoint
pub fn run_fuzzer(url: &str) {
    println!("⚡ Starting fuzzer for {}", url);
    engine::start_fuzzing(url);
}
//! Orchestrates the fuzzing loop and sends mutated requests

use crate::fuzzer::mutation::generate_mutations;
use crate::fuzzer::evaluator::evaluate_response;
use crate::fuzzer::strategy::{get_payloads, PayloadCategory};
use crate::fuzzer::templates::{get_form_template, get_json_template};
use reqwest::blocking::Client;
use std::time::Duration;

pub fn start_fuzzing(base_url: &str) {
    let client = Client::builder()
        .timeout(Duration::from_secs(5))
        .build()
        .unwrap();

    let base_payloads = get_payloads(PayloadCategory::All);
    let mutations = generate_mutations(&base_payloads.iter().map(|s| s.as_str()).collect::<Vec<_>>());

    for payload in mutations {
        // GET
        let target_url = format!("{}?q={}", base_url, urlencoding::encode(&payload));
        println!("[GET Fuzzing] {}", target_url);

        match client.get(&target_url).send() {
            Ok(resp) => {
                evaluate_response(&payload, &resp);
            }
            Err(e) => {
                println!("[Error] GET {} => {}", payload, e);
            }
        }

        // POST JSON
        let json = get_json_template(&payload);
        println!("[POST JSON] {}", base_url);
        let res = client.post(base_url)
            .header("Content-Type", json.content_type)
            .body(json.body.clone())
            .send();
        if let Ok(resp) = res {
            evaluate_response(&payload, &resp);
        }

        // POST Form
        let form = get_form_template(&payload);
        let res = client.post(base_url)
            .header("Content-Type", form.content_type)
            .body(form.body.clone())
            .send();
        if let Ok(resp) = res {
            evaluate_response(&payload, &resp);
        }
    }
}
