use std::collections::HashMap;
use regex::Regex;
use log::info;

#[derive(Debug, Clone)]
pub struct TokenData {
    pub name: String,
    pub value: String,
    pub source: String,
}

pub fn extract_tokens_from_headers(headers: &HashMap<String, String>) -> Vec<TokenData> {
    let mut tokens = Vec::new();

    for (key, value) in headers {
        if key.to_lowercase().contains("cookie") || key.to_lowercase().contains("authorization") {
            for pair in value.split(';') {
                if let Some((name, val)) = pair.trim().split_once('=') {
                    tokens.push(TokenData {
                        name: name.trim().to_string(),
                        value: val.trim().to_string(),
                        source: key.clone(),
                    });
                }
            }
        }
    }

    tokens
}

pub fn extract_tokens_from_burp_log(log: &str) -> Vec<TokenData> {
    let mut tokens = Vec::new();
    let cookie_regex = Regex::new(r"(?i)Set-Cookie: ([^=]+)=([^;\r\n]+)").unwrap();

    for cap in cookie_regex.captures_iter(log) {
        tokens.push(TokenData {
            name: cap[1].to_string(),
            value: cap[2].to_string(),
            source: "burp_log".to_string(),
        });
    }

    tokens
}
