use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub enum Error {
    BadRequestJson(::serde_json::Error),
    BadRequest,
    NoWindow,
    BadResponse,
    BadResponseJson(::serde_json::Error),
    EosError(ErrorResponse),
}

#[cfg(feature = "use-hyper")]
impl From<hyper::Error> for Error {
    fn from(err: hyper::Error) -> Self {
        println!("HYPER ERROR: {:#?}", err);
        Error::BadResponse
    }
}

impl From<serde_json::Error> for Error {
    fn from(err: serde_json::Error) -> Self {
        println!("SERDE ERROR: {:#?}", err);
        Error::BadResponse
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ErrorResponse {
    pub code: u16,
    pub message: String,
    pub error: ErrorMessage,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ErrorMessage {
    pub code: u16,
    pub name: String,
    pub what: String,
    pub details: Vec<ErrorDetails>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ErrorDetails {
    pub message: String,
    pub file: String,
    pub line_number: u32,
    pub method: String,
}
