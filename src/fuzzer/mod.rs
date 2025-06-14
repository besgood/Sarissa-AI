//! Entry point for fuzzing and mutation loop

pub mod engine;
pub mod mutation;
pub mod evaluator;

use crate::fuzzer::engine::start_fuzzing;

/// Kicks off fuzzing workflow on a target endpoint
pub fn run_fuzzer(url: &str) {
    println!("⚡ Starting fuzzer for {}", url);
    start_fuzzing(url);
}
