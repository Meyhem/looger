#[derive(Debug, Deserialize)]
pub struct Logger {
    pub name: String,
}

#[derive(Debug, Deserialize)]
pub struct User {
    pub login: String,
    pub password: String,
}

#[derive(Debug, Deserialize)]
pub struct ApplicationConfig {
    pub loggers: Vec<Logger>,
    pub users: Vec<User>,
}
