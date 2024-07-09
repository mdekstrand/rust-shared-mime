//! Support for parsing entries from the XDG Shared Mime Info database.
pub mod answer;
pub mod error;
mod fnmatch;
pub mod mimedb;
pub mod record;
#[cfg(feature = "xdg-runtime")]
pub mod runtime;

pub use error::LoadError;

pub use answer::Answer;
pub use mimedb::MimeDB;
