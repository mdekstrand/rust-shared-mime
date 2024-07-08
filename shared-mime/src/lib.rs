//! Support for parsing entries from the XDG Shared Mime Info database.
#[cfg(feature = "cli")]
pub mod cli;
mod record;
#[cfg(feature = "xdg-runtime")]
pub mod runtime;
