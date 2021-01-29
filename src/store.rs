use chrono::prelude::*;
use sled::{Db, IVec};

#[derive(Serialize, Deserialize, Debug)]
pub enum LogLevel {
    Verbose = 1,
    Debug,
    Info,
    Warn,
    Error,
    Fatal,
    Undefined,
}

impl From<String> for LogLevel {
    fn from(s: String) -> Self {
        match &s.to_ascii_lowercase()[..] {
            "verbose" => LogLevel::Verbose,
            "debug" => LogLevel::Debug,
            "info" => LogLevel::Info,
            "warn" => LogLevel::Warn,
            "error" => LogLevel::Error,
            "fatal" => LogLevel::Fatal,
            "unspecified" => LogLevel::Undefined,
            _ => LogLevel::Undefined,
        }
    }
}

impl From<&String> for LogLevel {
    fn from(s: &String) -> Self {
        LogLevel::from(s.clone())
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct StoredLog {
    id: [u8; 16],
    level: LogLevel,
    scope: String,
    message: String,
}

pub fn format_log_indetifier(id: u64) -> [u8; 16] {
    let lower: [u8; 8] = id.to_be_bytes();
    let upper: [u8; 8] = Utc::now().timestamp_nanos().to_be_bytes();

    let mut result = [0u8; 16];

    for i in 0..8 {
        result[i] = lower[i];
    }

    for i in 0..8 {
        result[i + 8] = upper[i];
    }

    result
}

pub fn new_stored_log(id: [u8; 16], level: &String, scope: String, message: String) -> StoredLog {
    StoredLog {
        id,
        level: LogLevel::from(level),
        scope,
        message,
    }
}

pub fn store_batch(logs: &Vec<StoredLog>, db: &Db) -> std::result::Result<(), sled::Error> {
    let mut batch = sled::Batch::default();

    for log in logs {
        let bin = bincode::serialize(&log).unwrap();
        batch.insert(IVec::from(&log.id), IVec::from(bin));
    }

    db.apply_batch(batch)
}
