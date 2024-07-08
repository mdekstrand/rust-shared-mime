//! Support for parsing entries from the XDG Shared Mime Info database.
mod mime_type;
#[cfg(feature = "xdg-runtime")]
pub mod runtime;
