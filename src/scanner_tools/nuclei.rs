
use std::process::Command;

pub fn run_nuclei_scan(target: &str) {
    println!("[Nuclei] Scanning: {}", target);
    let output = Command::new("nuclei")
        .args(["-u", target, "-t", "cves/", "-o", "nuclei_results.txt"])
        .output();

    match output {
        Ok(o) => println!("{}", String::from_utf8_lossy(&o.stdout)),
        Err(e) => eprintln!("Failed to run nuclei: {}", e),
    }
}
