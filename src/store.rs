use std::{
    convert::TryInto,
    ops::{Add, Deref},
};

use chrono::prelude::*;
use sled::{Db, IVec};

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
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

impl Into<String> for LogLevel {
    fn into(self) -> String {
        match self {
            LogLevel::Verbose => "Verbose",
            LogLevel::Debug => "Debug",
            LogLevel::Info => "Info",
            LogLevel::Warn => "Warn",
            LogLevel::Error => "Error",
            LogLevel::Fatal => "Fatal",
            LogLevel::Undefined => "Undefined",
        }
        .to_owned()
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct StoredLog {
    pub id: [u8; 16],
    pub level: LogLevel,
    pub scope: String,
    pub message: String,
}

pub fn format_log_identifier(id: u64) -> [u8; 16] {
    let lower: [u8; 8] = id.to_be_bytes();
    let upper: [u8; 8] = Utc::now().timestamp_nanos().to_be_bytes();

    let mut result = [0u8; 16];

    for i in 0..8 {
        result[i] = upper[i];
    }

    for i in 0..8 {
        result[i + 8] = lower[i];
    }

    result
}

pub fn parse_log_identifier(bin: &[u8]) -> Option<(DateTime<Utc>, u64)> {
    let sized_stamp: Result<[u8; 8], _> = bin[0..8].try_into();
    let sized_id: Result<[u8; 8], _> = bin[8..16].try_into();

    if sized_stamp.is_err() || sized_id.is_err() {
        return None;
    }

    let stamp = u64::from_be_bytes(sized_stamp.unwrap()) as i64;
    let id = u64::from_be_bytes(sized_id.unwrap());

    let datetime = Utc
        .timestamp(0, 0)
        .add(chrono::Duration::nanoseconds(stamp));

    Some((datetime, id))
}

pub enum Bound {
    Lower,
    Upper,
}

fn format_log_identifier_for_bound(stamp: DateTime<Utc>, bound: Bound) -> [u8; 16] {
    let upper = stamp.timestamp_nanos().to_be_bytes();
    let mut result = [0u8; 16];
    for i in 0..8 {
        result[i] = upper[i];
    }

    let fill = match bound {
        Bound::Lower => 0x00u8,
        Bound::Upper => 0xFFu8,
    };

    for i in 0..8 {
        result[i + 8] = fill;
    }

    result
}

pub fn query(
    db: &Db,
    from: DateTime<Utc>,
    to: DateTime<Utc>,
    offset: usize,
    limit: usize,
) -> Vec<StoredLog> {
    let lower_bound = format_log_identifier_for_bound(from, Bound::Lower);
    let upper_bound = format_log_identifier_for_bound(to, Bound::Upper);

    let records: Vec<StoredLog> = db
        .range(lower_bound..upper_bound)
        .skip(offset)
        .take(limit)
        .filter(|item| item.is_ok())
        .map(|item| item.unwrap())
        .filter_map(|(_, binlog)| bincode::deserialize::<StoredLog>(binlog.deref()).ok())
        .collect();

    records
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
