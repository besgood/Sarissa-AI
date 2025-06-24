use std::process::Command;

pub fn launch_sliver_listener() {
    println!("[Sliver] Launching Sliver C2 listener...");
    let output = Command::new("sliver-server")
        .args(["--lhost", "0.0.0.0", "--lport", "31337"])
        .output();

    match output {
        Ok(o) => println!("[Sliver Output]\n{}", String::from_utf8_lossy(&o.stdout)),
        Err(e) => eprintln!("Failed to start Sliver: {}", e),
    }
}
