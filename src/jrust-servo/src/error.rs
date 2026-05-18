
//! Error type definitions

use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Snap load failed: {0}")]
    SnapLoadError(String),

    #[error("Snap conversion failed: {0}")]
    SnapConversionError(String),

    #[error("Servo init failed: {0}")]
    ServoInitError(String),

    #[error("Event send failed: {0}")]
    EventSendError(String),

    #[error("DOM update failed: {0}")]
    DomUpdateError(String),

    #[error("General error: {0}")]
    GeneralError(String),

    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("JSON error: {0}")]
    JsonError(#[from] serde_json::Error),
}

impl From<String> for Error {
    fn from(err: String) -> Self {
        Error::GeneralError(err)
    }
}

impl From<&str> for Error {
    fn from(err: &str) -> Self {
        Error::GeneralError(err.to_string())
    }
}
