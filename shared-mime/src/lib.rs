//! Support for parsing entries from the XDG Shared Mime Info database.
pub mod answer;
mod fnmatch;
pub mod mimedb;
pub mod record;
#[cfg(feature = "xdg-runtime")]
pub mod runtime;
