use failure::Fail;
use std::convert::From;
use reqwest;
use url::ParseError;
use std::env::VarError;

#[derive(Debug, Fail)]
pub enum GpiError {
    #[fail(display = "UnknownVcs: {}", _0)]
    UnknownVcs(String),
    #[fail(display = "UnknownPackageType: {}", _0)]
    UnknownPackageType(String),
    #[fail(display = "{}", _0)]
    JsonError(#[fail(cause)] serde_json::Error),
    #[fail(display="{}",_0)]
    ConversionError(String),
    #[fail(display = "{}", _0)]
    MissingPackage(String),
    #[fail(display = "{}", _0)]
    FailureError(#[fail(cause)] failure::Error),
    #[fail(display = "Lookup error for {}: {}", _0, _1)]
    PackageLookupError(String, String),
    #[fail(display = "{}", _0)]
    ReqwestError(#[fail(cause)] reqwest::Error),
    #[fail(display = "{}", _0)]
    UrlParseError(#[fail(cause)] ParseError),
    #[fail(display = "{}", _0)]
    VarError(#[fail(cause)] VarError),
}

impl From<serde_json::Error> for GpiError {
    fn from(value: serde_json::Error) -> Self {
        GpiError::JsonError(value)
    }
}

impl From<VarError> for GpiError {
    fn from(value: VarError) -> Self {
        GpiError::VarError(value)
    }
}

impl From<reqwest::Error> for GpiError {
    fn from(value: reqwest::Error) -> Self {
        GpiError::ReqwestError(value)
    }
}

impl From<ParseError> for GpiError {
    fn from(value: ParseError) -> Self {
        GpiError::UrlParseError(value)
    }
}

impl From<failure::Error> for GpiError {
   fn from(e: failure::Error) -> GpiError {
       GpiError::FailureError(e)
    }
}

impl From<&str> for GpiError {
   fn from(e: &str) -> GpiError {
       GpiError::ConversionError(e.to_string())
    }
}