
use std::process::Command;

pub fn run_ffuf(target: &str, wordlist: &str) {
    println!("[FFUF] Fuzzing: {}", target);
    let output = Command::new("ffuf")
        .args(["-u", &format!("{}/FUZZ", target), "-w", wordlist])
        .output();

    match output {
        Ok(o) => println!("{}", String::from_utf8_lossy(&o.stdout)),
        Err(e) => eprintln!("FFUF error: {}", e),
    }
}
