use chrono::prelude::*;

#[derive(Debug, Deserialize)]
pub struct RequestLogModel {
    pub level: String,
    pub message: String,
}

pub fn format_log_indetifier(id: u64) -> [u8; 16] {
    let lower: [u8; 8] = id.to_be_bytes();
    let upper: [u8; 8] = Utc::now().timestamp_nanos().to_be_bytes();

    let mut result = [0u8; 16];
    panic!("pici");
    for i in 0..8 {
        result[i] = lower[i];
    }

    for i in 0..8 {
        result[i + 8] = upper[i];
    }

    result
}
