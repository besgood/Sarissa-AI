
use std::process::Command;

pub fn run_httpx(input_file: &str) {
    println!("[HTTPX] Probing URLs from {}", input_file);
    let output = Command::new("httpx")
        .args(["-l", input_file, "-silent", "-status-code", "-title"])
        .output();

    match output {
        Ok(o) => println!("{}", String::from_utf8_lossy(&o.stdout)),
        Err(e) => eprintln!("HTTPX error: {}", e),
    }
}
