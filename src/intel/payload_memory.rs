// src/intel/payload_memory.rs

use std::collections::HashMap;
use std::fs::{self, File};
use std::io::{BufReader, BufWriter};
use std::path::Path;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PayloadRecord {
    pub payload: String,
    pub category: String,
    pub status: String, // "blocked", "bypassed", "error"
    pub waf_response: Option<String>,
}

#[derive(Serialize, Deserialize, Default)]
pub struct PayloadHistory {
    pub records: Vec<PayloadRecord>,
}

impl PayloadHistory {
    pub fn add_record(&mut self, record: PayloadRecord) {
        self.records.push(record);
    }

    pub fn save(&self, path: &str) {
        if let Ok(file) = File::create(path) {
            let writer = BufWriter::new(file);
            serde_json::to_writer_pretty(writer, &self).ok();
        }
    }

    pub fn load(path: &str) -> Self {
        if Path::new(path).exists() {
            if let Ok(file) = File::open(path) {
                let reader = BufReader::new(file);
                if let Ok(history) = serde_json::from_reader(reader) {
                    return history;
                }
            }
        }
        PayloadHistory::default()
    }

    pub fn filter_by_status(&self, status: &str) -> Vec<&PayloadRecord> {
        self.records.iter().filter(|r| r.status == status).collect()
    }
}
