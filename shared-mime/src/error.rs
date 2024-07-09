//! Shared MIME DB errors.

use std::io;

use thiserror::Error;

/// Errors that can occur when loading the MIME data.
#[derive(Debug, Error)]
pub enum LoadError {
    #[cfg(feature = "xdg-runtime")]
    #[error("XDG load error: {0}")]
    XDG(#[from] crate::runtime::XDGError),

    #[error("MIME database unavailable")]
    Unavailable,

    #[error("load error: {0}")]
    Generic(String),
}

/// Create a load error with the specified message.
pub fn load_error<S: AsRef<str>>(msg: S) -> LoadError {
    LoadError::Generic(msg.as_ref().into())
}

/// Errors that can occur when querying the MIME data.
#[derive(Debug, Error)]
pub enum QueryError {
    #[error("load error: {0}")]
    Generic(String),
    #[error("I/O error: {0}")]
    IO(#[from] io::Error),
}

/// Create a load error with the specified message.
pub fn query_error<S: AsRef<str>>(msg: S) -> QueryError {
    QueryError::Generic(msg.as_ref().into())
}
