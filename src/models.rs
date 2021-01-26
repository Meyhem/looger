use chrono::{DateTime, Utc};

#[derive(Deserialize)]
pub struct RequestAppendLog {
  pub message: String,
  pub timestamp: DateTime<Utc>,
}
