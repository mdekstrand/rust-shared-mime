//! Shared MIME DB errors.

use thiserror::Error;

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
