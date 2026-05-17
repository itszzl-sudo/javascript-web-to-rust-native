use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Parse error at {location}: {message}")]
    ParseError {
        location: String,
        message: String,
    },

    #[error("Compilation error: {0}")]
    CompilationError(String),

    #[error("File too large: {size} bytes (max: {max} bytes)")]
    FileTooLarge {
        size: usize,
        max: usize,
    },

    #[error("Recursion limit exceeded (max depth: {max})")]
    RecursionLimitExceeded { max: usize },

    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
}

pub type Result<T> = std::result::Result<T, Error>;
