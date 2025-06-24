
use std::process::Command;

pub fn run_gowitness(target_file: &str) {
    println!("[Gowitness] Screenshotting URLs from {}", target_file);
    let output = Command::new("gowitness")
        .args(["file", "-f", target_file])
        .output();

    match output {
        Ok(o) => println!("{}", String::from_utf8_lossy(&o.stdout)),
        Err(e) => eprintln!("Gowitness error: {}", e),
    }
}
