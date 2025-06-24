use std::process::Command;

pub fn launch_cobalt_strike_teamserver(ip: &str, password: &str) {
    println!("[CobaltStrike] Launching teamserver on {}...", ip);
    let output = Command::new("bash")
        .arg("-c")
        .arg(format!("./teamserver {} {}", ip, password))
        .output();

    match output {
        Ok(o) => println!("[CobaltStrike Output]\n{}", String::from_utf8_lossy(&o.stdout)),
        Err(e) => eprintln!("Failed to launch Cobalt Strike: {}", e),
    }
}
