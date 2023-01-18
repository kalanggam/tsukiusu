use std::error::Error;

// DictError
// ---------
// UnsupportedLang: language not supported
// HttpError: reqwest crate error
// ParseError: url parser error
#[derive(Debug)]
pub enum DictError {
    UnsupportedLang(String),
    HttpError(reqwest::Error),
    ResponseError(reqwest::Error),
    ParseError(url::ParseError),
}

// Implement error trait
impl Error for DictError {}

// Implement Display
impl std::fmt::Display for DictError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Dictionary lookup error - ")?;
        match self {
            DictError::UnsupportedLang(s) => write!(f, "unsupported language {}", s),
            DictError::HttpError(e) => e.fmt(f),
            DictError::ResponseError(e) => e.fmt(f),
            DictError::ParseError(e) => e.fmt(f),
        }
    }
}