//! Performs passive reconnaissance (headers, DNS, TLS)

use reqwest::blocking::Client;
use std::collections::HashMap;
use std::net::ToSocketAddrs;
use std::time::Duration;

pub fn passive_recon(target: &str) -> HashMap<String, String> {
    let mut info = HashMap::new();
    let client = Client::builder()
        .timeout(Duration::from_secs(5))
        .build()
        .unwrap();

    let url = if target.starts_with("http") {
        target.to_string()
    } else {
        format!("http://{}", target)
    };

    // DNS
    if let Ok(addrs) = (target, 80).to_socket_addrs() {
        let ips: Vec<String> = addrs.map(|a| a.ip().to_string()).collect();
        info.insert("DNS IPs".to_string(), ips.join(", "));
    }

    // HTTP Headers
    if let Ok(resp) = client.get(&url).send() {
        for (key, value) in resp.headers().iter() {
            if let Ok(val) = value.to_str() {
                info.insert(format!("Header: {}", key), val.to_string());
            }
        }
    } else {
        info.insert("Error".to_string(), "Connection failed".to_string());
    }

    info
}
