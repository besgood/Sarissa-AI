
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

#[derive(Debug, Clone)]
pub struct SessionData {
    pub cookies: HashMap<String, String>,
    pub headers: HashMap<String, String>,
    pub tokens: Vec<String>,
}

#[derive(Clone)]
pub struct SessionMemory {
    store: Arc<Mutex<HashMap<String, SessionData>>>,
}

impl SessionMemory {
    pub fn new() -> Self {
        SessionMemory {
            store: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub fn save_session(&self, key: &str, data: SessionData) {
        self.store.lock().unwrap().insert(key.to_string(), data);
    }

    pub fn get_session(&self, key: &str) -> Option<SessionData> {
        self.store.lock().unwrap().get(key).cloned()
    }

    pub fn update_cookies(&self, key: &str, new_cookies: HashMap<String, String>) {
        let mut store = self.store.lock().unwrap();
        if let Some(session) = store.get_mut(key) {
            session.cookies.extend(new_cookies);
        }
    }

    pub fn add_token(&self, key: &str, token: String) {
        let mut store = self.store.lock().unwrap();
        if let Some(session) = store.get_mut(key) {
            session.tokens.push(token);
        }
    }
}
