//! Proxy integration entry point for Burp/ZAP support

pub mod burp_parser;
pub mod replay;
pub mod auth;

use crate::proxy::burp_parser::BurpRequest;
use crate::proxy::replay::replay_requests;
use crate::proxy::auth::SessionContext;

/// Process captured Burp traffic and invoke AI logic
pub fn analyze_burp_log(file_path: &str, session: Option<SessionContext>) {
    match burp_parser::parse_burp_file(file_path) {
        Ok(requests) => {
            println!("Loaded {} Burp requests", requests.len());
            replay_requests(&requests, session);
        }
        Err(e) => {
            eprintln!("Failed to parse Burp log: {}", e);
        }
    }
}
