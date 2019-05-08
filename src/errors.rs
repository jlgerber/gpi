use failure::Fail;
use std::convert::From;

#[derive(Debug, Fail)]
pub enum GpiError {
    #[fail(display = "UnknownVcs: {}", _0)]
    UnknownVcs(String),
    #[fail(display = "{}", _0)]
    JsonError(#[fail(cause)] serde_json::Error),
    #[fail(display = "{}", _0)]
    FailureError(#[fail(cause)] failure::Error),
}

impl From<serde_json::Error> for GpiError {
    fn from(value: serde_json::Error) -> Self {
        GpiError::JsonError(value)
    }
}

impl From<failure::Error> for GpiError {
   fn from(e: failure::Error) -> GpiError {
       GpiError::FailureError(e)
    }
}