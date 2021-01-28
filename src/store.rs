pub enum LogLevel {
    Verbose = 1,
    Debug,
    Info,
    Warn,
    Error,
    Fatal,
}

pub struct StoredLog {
    level: LogLevel,
    message: String,
}
