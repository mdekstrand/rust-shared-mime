//! Support for parsing entries from the XDG Shared Mime Info database.
pub mod record;
#[cfg(feature = "xdg-runtime")]
pub mod runtime;
