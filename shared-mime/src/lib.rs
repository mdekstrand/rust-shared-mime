//! Support for parsing entries from the XDG Shared Mime Info database.
mod fnmatch;
pub mod mimedb;
pub mod record;
#[cfg(feature = "xdg-runtime")]
pub mod runtime;
