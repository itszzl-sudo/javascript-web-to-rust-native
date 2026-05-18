
//! Error type definitions

use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Browser init failed: {0}")]
    BrowserInitError(String),

    #[error("DOM operation failed: {0}")]
    DomOperationError(String),

    #[error("Render failed: {0}")]
    RenderError(String),

    #[error("Event handling failed: {0}")]
    EventError(String),

    #[error("Bridge error: {0}")]
    BridgeError(String),

    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("JSON error: {0}")]
    JsonError(#[from] serde_json::Error),
}

impl From<String> for Error {
    fn from(err: String) -> Self {
        Error::BridgeError(err)
    }
}

impl From<&str> for Error {
    fn from(err: &str) -> Self {
        Error::BridgeError(err.to_string())
    }
}
