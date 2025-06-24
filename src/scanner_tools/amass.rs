
use std::process::Command;

pub fn run_amass(domain: &str) {
    println!("[Amass] Enum for domain: {}", domain);
    let output = Command::new("amass")
        .args(["enum", "-d", domain])
        .output();

    match output {
        Ok(o) => println!("{}", String::from_utf8_lossy(&o.stdout)),
        Err(e) => eprintln!("Amass error: {}", e),
    }
}
