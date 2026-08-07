// shared/errors.rs
//
// Application-wide error types.
// Every layer uses these. No layer defines its own error types.

use thiserror::Error;

#[derive(Debug, Error)]
pub enum SanchayaError {
    #[error("Database error: {0}")]
    Database(#[from] rusqlite::Error),

    #[error("Document not found: {0}")]
    NotFound(String),

    #[error("Validation error: {0}")]
    Validation(String),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
}

// Every use case and command returns this Result type.
pub type Result<T> = std::result::Result<T, SanchayaError>;
