use std::error::Error;

#[derive(Debug)]
pub struct Rfc3339DateTime(chrono::DateTime<chrono::Utc>);

#[derive(Debug)]
pub enum DateTimeParseError {
    DecodeError,
    ParseError(String),
}

impl std::fmt::Display for DateTimeParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

impl Error for DateTimeParseError {}

impl<'v> rocket::request::FromFormValue<'v> for Rfc3339DateTime {
    type Error = DateTimeParseError;

    fn from_form_value(form_value: &'v rocket::http::RawStr) -> Result<Self, Self::Error> {
        match form_value.url_decode() {
            Ok(decoded) => chrono::DateTime::parse_from_rfc3339(&decoded[..])
                .map(|v| Rfc3339DateTime(v.into()))
                .map_err(|e| DateTimeParseError::ParseError(e.to_string())),

            Err(_) => Err(DateTimeParseError::DecodeError),
        }
    }
}
