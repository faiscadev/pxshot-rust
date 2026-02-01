//! Error types for the Pxshot SDK.

use thiserror::Error;

/// Errors that can occur when using the Pxshot SDK.
#[derive(Error, Debug)]
pub enum Error {
    /// The request is missing required fields.
    #[error("missing required field: {0}")]
    MissingField(&'static str),

    /// HTTP request failed.
    #[error("HTTP request failed: {0}")]
    Request(#[from] reqwest::Error),

    /// API returned an error response.
    #[error("API error ({status}): {message}")]
    Api {
        /// HTTP status code.
        status: u16,
        /// Error message from the API.
        message: String,
    },

    /// Failed to parse API response.
    #[error("failed to parse response: {0}")]
    Parse(String),

    /// Invalid configuration.
    #[error("invalid configuration: {0}")]
    Config(String),
}

/// Result type alias using the Pxshot error type.
pub type Result<T> = std::result::Result<T, Error>;
