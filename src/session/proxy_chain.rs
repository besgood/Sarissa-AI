use crate::proxy::session_store::{SessionData, SessionStore};
use crate::ai::query_llm;
use log::{info, warn};
use reqwest::blocking::{Client, RequestBuilder};
use reqwest::header::{HeaderMap, HeaderName, HeaderValue};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ProxyStep {
    pub method: String,
    pub url: String,
    pub headers: Option<HashMap<String, String>>,
    pub body: Option<String>,
}

pub fn replay_proxy_chain(chain: &[ProxyStep], session_path: &str, use_ai: bool) {
    let client = Client::new();
    let session_store = SessionStore::new(session_path);
    let mut session_data = session_store.load();

    for (i, step) in chain.iter().enumerate() {
        info!("[Chain Replay] Step {}: {}", i + 1, step.url);

        let mut builder = match step.method.to_uppercase().as_str() {
            "GET" => client.get(&step.url),
            "POST" => client.post(&step.url),
            "PUT" => client.put(&step.url),
            "DELETE" => client.delete(&step.url),
            _ => {
                warn!("Unsupported method: {}", step.method);
                continue;
            }
        };

        // Set headers from session + step
        let mut headers = HeaderMap::new();
        for (k, v) in &session_data.headers {
            if let (Ok(key), Ok(val)) = (HeaderName::from_bytes(k.as_bytes()), HeaderValue::from_str(v)) {
                headers.insert(key, val);
            }
        }
        if let Some(step_headers) = &step.headers {
            for (k, v) in step_headers {
                if let (Ok(key), Ok(val)) = (HeaderName::from_bytes(k.as_bytes()), HeaderValue::from_str(v)) {
                    headers.insert(key, val);
                }
            }
        }
        builder = builder.headers(headers);

        // Optionally mutate body
        let mut body = step.body.clone();
        if use_ai {
            if let Some(content) = &body {
                let prompt = format!("Modify this request body for bypass or fuzzing:\n{}", content);
                let ai_response = query_llm("Improve HTTP body payload", &prompt);
                body = Some(ai_response);
            }
        }

        if let Some(content) = body {
            builder = builder.body(content);
        }

        let response = builder.send();
        match response {
            Ok(resp) => {
                info!("→ Status: {}", resp.status());
                if let Ok(text) = resp.text() {
                    println!("→ Response (truncated): {}", &text[..text.len().min(200)]);
                }
            }
            Err(e) => warn!("Request failed: {}", e),
        }
    }
}
