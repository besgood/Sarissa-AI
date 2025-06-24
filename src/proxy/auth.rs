//! Manages session headers (e.g., cookies, auth tokens)

use reqwest::header::{HeaderMap, HeaderName, HeaderValue};
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct SessionContext {
    pub custom_headers: HashMap<HeaderName, HeaderValue>,
}

impl SessionContext {
    /// Creates a context with common auth headers (e.g., from CLI input)
    pub fn from_kv_pairs(pairs: Vec<(&str, &str)>) -> Self {
        let mut headers = HashMap::new();
        for (k, v) in pairs {
            if let (Ok(hn), Ok(hv)) = (
                HeaderName::from_bytes(k.as_bytes()),
                HeaderValue::from_str(v)
            ) {
                headers.insert(hn, hv);
            }
        }
        SessionContext { custom_headers: headers }
    }

    /// Example: hardcoded Bearer token
    pub fn with_bearer_token(token: &str) -> Self {
        Self::from_kv_pairs(vec![
            ("Authorization", &format!("Bearer {}", token)),
        ])
    }
}
