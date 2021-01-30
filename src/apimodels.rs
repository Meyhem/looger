use chrono::{DateTime, TimeZone, Utc};

use crate::store::{parse_log_identifier, StoredLog};

#[derive(Debug, Deserialize)]
pub struct RequestLogModel {
    pub level: String,
    pub scope: String,
    pub message: String,
}

#[derive(Debug, Serialize)]
pub struct QueryLogModel {
    pub id: u64,
    pub timestamp: DateTime<Utc>,
    pub level: String,
    pub scope: String,
    pub message: String,
}

impl From<&StoredLog> for QueryLogModel {
    fn from(l: &StoredLog) -> Self {
        let (timestamp, id) = parse_log_identifier(&l.id[..])
            .or(Some((Utc.ymd(1970, 1, 1).and_hms(0, 0, 0), 0)))
            .unwrap();

        QueryLogModel {
            id,
            timestamp,
            level: l.level.into(),
            message: l.message.clone(),
            scope: l.scope.clone(),
        }
    }
}
