#[derive(Debug, Deserialize)]
pub struct Logger {
  pub name: String,
}

#[derive(Debug, Deserialize)]
pub struct ApplicationConfig {
  pub loggers: Vec<Logger>,
}
