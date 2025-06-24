use std::process::Command;

pub fn start_mythic_services() {
    println!("[Mythic] Starting Mythic C2 container stack...");
    let output = Command::new("bash")
        .arg("-c")
        .arg("cd ~/Mythic && ./mythic-cli start")
        .output();

    match output {
        Ok(o) => println!("[Mythic Output]\n{}", String::from_utf8_lossy(&o.stdout)),
        Err(e) => eprintln!("Failed to start Mythic: {}", e),
    }
}
