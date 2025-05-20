use crate::prelude::*;
use std::fmt;

/// This module contains the definitions for BybitContentError and BybitError, two custom error types.
/// BybitContentError is a struct that represents the error returned by the Bybit API, and BybitError is
/// an enum that can hold any possible error that can occur during the execution of the program.

/// BybitContentError is a struct that represents the error returned by the Bybit API.
/// It has two fields: code, which is an i16 representing the error code, and msg, which is a String
/// representing the error message.
#[derive(Debug, Deserialize)]
pub struct BybitContentError {
    pub code: i32,
    pub msg: String,
}

/// BybitError is an enum that can hold any possible error that can occur during the execution of the program.
/// It has several variants, each representing a different type of error.
#[derive(Debug, Error)]
pub enum BybitError {
    /// BybitError variant that holds a BybitContentError. This is used when the error returned by the Bybit API
    /// is of the type BybitContentError.
    #[error("Bybit error: {0}")]
    BybitError(BybitContentError),

    #[error("Failed to emit value on channel, underlying: {underlying}")]
    ChannelSendError { underlying: String },

    /// KlineValueMissingError variant that holds the index of the missing value, and the name of the missing value.
    /// This variant is used when a value in a kline vector is missing.
    #[error("Invalid Vec for Kline: {name} at {index} is missing")]
    KlineValueMissingError { index: usize, name: &'static str },

    /// Variants that hold the error returned by reqwest, serde_json, tokio_tungstenite, and std libraries.
    /// These variants are used when the respective library returns an error.
    #[error(transparent)]
    ReqError(#[from] reqwest::Error),

    #[error(transparent)]
    InvalidHeaderError(#[from] reqwest::header::InvalidHeaderValue),

    #[error(transparent)]
    IoError(#[from] std::io::Error),

    #[error(transparent)]
    ParseFloatError(#[from] std::num::ParseFloatError),

    #[error(transparent)]
    UrlParserError(#[from] url::ParseError),

    #[error(transparent)]
    Json(#[from] serde_json::Error),

    #[error(transparent)]
    Tungstenite(#[from] tokio_tungstenite::tungstenite::Error),

    #[error(transparent)]
    TimestampError(#[from] std::time::SystemTimeError),

    #[error(transparent)]
    SerdeError(#[from] serde::de::value::Error),

    // Variants representing common errors.
    #[error("Internal Server Error")]
    InternalServerError,

    #[error("Service Unavailable")]
    ServiceUnavailable,

    #[error("Unauthorized")]
    Unauthorized,

    /// StatusCode variant that holds the status code.
    #[error("Status Code")]
    StatusCode(u16),

    /// Base variant that holds a String representing the error.
    /// This variant is used when the error is not of any specific type, and it is just a simple String.
    #[error("Bybit error: {0}")]
    Base(String),
}

// Implement the fmt::Display trait for BybitContentError.
// This trait is used to specify how BybitContentError should be converted to a string.
impl fmt::Display for BybitContentError {
    // This method takes a mutable reference to a fmt::Formatter, and returns a fmt::Result.
    // It writes the `msg` field of the BybitContentError struct to the given formatter.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Implement this method to dictate how BybitContentError should be converted to a string.
        // This is a placeholder implementation that simply writes the `msg` field; you should
        // replace it with your own format string.
        write!(f, "{}", self.msg)
    }
}

// Implement the From trait for String and BybitError.
// This trait is used to specify how a String can be converted to BybitError.
impl From<std::string::String> for BybitError {
    // This function takes a String, and returns a BybitError.
    fn from(err: String) -> Self {
        //
        // Convert the String error to BybitError here
        // For example, you can return a new instance of BybitError with the error message
        BybitError::new(err)
    }
}

impl BybitError {
    fn new(arg: String) -> Self {
        BybitError::Base(arg)
    }
}
