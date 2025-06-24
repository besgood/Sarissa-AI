// src/reputation/mod.rs

pub mod scoring;
pub mod reputation_db;

pub use scoring::{calculate_reputation, ReputationScore};
pub use reputation_db::{ReputationEntry, ReputationDB};
