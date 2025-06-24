// src/reputation/reputation_db.rs

use crate::reputation::scoring::ReputationScore;
use std::fs::{self, OpenOptions};
use std::io::{BufReader, BufWriter};
use std::path::Path;
use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use std::io::prelude::*;

const DB_FILE: &str = "reputation_scores.json";

#[derive(Serialize, Deserialize, Debug)]
pub struct ReputationDatabase {
    pub scores: HashMap<String, ReputationScore>,
}

impl ReputationDatabase {
    pub fn load() -> Self {
        if Path::new(DB_FILE).exists() {
            let file = OpenOptions::new().read(true).open(DB_FILE).unwrap();
            let reader = BufReader::new(file);
            serde_json::from_reader(reader).unwrap_or_else(|_| ReputationDatabase {
                scores: HashMap::new(),
            })
        } else {
            ReputationDatabase {
                scores: HashMap::new(),
            }
        }
    }

    pub fn save(&self) {
        let file = OpenOptions::new().write(true).create(true).truncate(true).open(DB_FILE).unwrap();
        let writer = BufWriter::new(file);
        serde_json::to_writer_pretty(writer, self).unwrap();
    }

    pub fn update(&mut self, score: ReputationScore) {
        self.scores.insert(score.target.clone(), score);
        self.save();
    }

    pub fn get(&self, target: &str) -> Option<&ReputationScore> {
        self.scores.get(target)
    }

    pub fn list_all(&self) -> Vec<&ReputationScore> {
        self.scores.values().collect()
    }
}
