
use std::process::Command;

pub fn run_enum4linux(ip: &str) {
    println!("[enum4linux-ng] Enumerating SMB on {}", ip);
    let output = Command::new("enum4linux-ng")
        .args(["-A", ip])
        .output();

    match output {
        Ok(o) => println!("{}", String::from_utf8_lossy(&o.stdout)),
        Err(e) => eprintln!("enum4linux-ng error: {}", e),
    }
}
