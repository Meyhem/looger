#[derive(Debug, Deserialize)]
pub struct RequestLogModel {
    pub level: String,
    pub scope: String,
    pub message: String,
}
