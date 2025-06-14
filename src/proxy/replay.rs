//! Replay captured HTTP requests using reqwest

use crate::proxy::burp_parser::BurpRequest;
use crate::proxy::auth::SessionContext;
use reqwest::blocking::Client;
use reqwest::header::{HeaderMap, HeaderName, HeaderValue};

pub fn replay_requests(requests: &[BurpRequest], session: Option<SessionContext>) {
    let client = Client::new();

    for req in requests {
        let mut headers = HeaderMap::new();

        for h in &req.headers {
            if let Some((name, value)) = h.split_once(":") {
                if let (Ok(header_name), Ok(header_value)) = (
                    HeaderName::from_bytes(name.trim().as_bytes()),
                    HeaderValue::from_str(value.trim())
                ) {
                    headers.insert(header_name, header_value);
                }
            }
        }

        if let Some(sess) = &session {
            for (k, v) in &sess.custom_headers {
                headers.insert(k.clone(), v.clone());
            }
        }

        let res = match req.method.as_str() {
            "POST" => client.post(&req.url).headers(headers).body(req.body.clone().unwrap_or_default()).send(),
            "PUT" => client.put(&req.url).headers(headers).body(req.body.clone().unwrap_or_default()).send(),
            "DELETE" => client.delete(&req.url).headers(headers).send(),
            _ => client.get(&req.url).headers(headers).send(),
        };

        match res {
            Ok(response) => {
                println!("[{}] {} - Status: {}", req.method, req.url, response.status());
            }
            Err(e) => {
                eprintln!("Failed to send request to {}: {}", req.url, e);
            }
        }
    }
}
