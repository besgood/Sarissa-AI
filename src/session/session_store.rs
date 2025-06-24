use std::collections::HashMap;
use std::fs::{self, File};
use std::io::{BufReader, BufWriter};
use std::path::Path;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct SessionData {
    pub cookies: HashMap<String, String>,
    pub headers: HashMap<String, String>,
    pub tokens: HashMap<String, String>,
}

pub struct SessionStore {
    path: String,
}

impl SessionStore {
    pub fn new(path: &str) -> Self {
        Self { path: path.to_string() }
    }

    pub fn load(&self) -> SessionData {
        let file_path = Path::new(&self.path);
        if !file_path.exists() {
            return SessionData::default();
        }

        let file = File::open(&file_path).expect("Unable to open session store");
        let reader = BufReader::new(file);
        serde_json::from_reader(reader).unwrap_or_default()
    }

    pub fn save(&self, data: &SessionData) {
        let file = File::create(&self.path).expect("Unable to create session store");
        let writer = BufWriter::new(file);
        serde_json::to_writer_pretty(writer, data).expect("Unable to write session data");
    }

    pub fn clear(&self) {
        if Path::new(&self.path).exists() {
            fs::remove_file(&self.path).expect("Unable to delete session store file");
        }
    }
}
