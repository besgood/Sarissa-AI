
use std::process::Command;

pub fn run_rustscan(target: &str) {
    println!("[Rustscan] Target: {}", target);
    let output = Command::new("rustscan")
        .args(["-a", target])
        .output();

    match output {
        Ok(o) => println!("{}", String::from_utf8_lossy(&o.stdout)),
        Err(e) => eprintln!("Rustscan error: {}", e),
    }
}
