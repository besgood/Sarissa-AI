
use std::process::Command;

pub fn run_gospider(url: &str) {
    println!("[Gospider] Crawling: {}", url);
    let output = Command::new("gospider")
        .args(["-s", url, "-o", "gospider_out"])
        .output();

    match output {
        Ok(o) => println!("{}", String::from_utf8_lossy(&o.stdout)),
        Err(e) => eprintln!("Gospider error: {}", e),
    }
}
