//! Load XDG data files at runtime.
//!
//! This module provides support for loading the [XDG Shared Mime Info][SMI]
//! database at runtime from either the XDG directories or from specified
//! package files.
//!
//! This code directly parses the XML from the `packages/` directory, instead of
//! the pre-parsed files created by `update-mime-info`, for a few reasons:
//!
//! - It's pretty fast with [quick_xml].
//! - There are some weirdnesses in the `globs2` file on my machines around
//!   case-sensitive matches that are not in the original source.
//! - The same parsing code can directly load the XML from the
//!   `shared-mime-info` source repository for embedding.
//!
//! [SMI]:
//!     https://specifications.freedesktop.org/shared-mime-info-spec/shared-mime-info-spec-latest.html
mod dirs;
pub mod mimeinfo;
pub mod xdg_package;
mod xdg_parse;

use std::io;

use quick_xml::DeError;
use thiserror::Error;

pub use dirs::xdg_mime_search_dirs;
pub use mimeinfo::load_xdg_mime_info;
pub use xdg_parse::parse_mime_package;

/// Error type for mime-info parse failures.
#[derive(Error, Debug)]
pub enum XDGError {
    #[error("I/O error: {0}")]
    IO(#[from] io::Error),
    #[error("XML deserialize error: {0}")]
    Deserialize(#[from] DeError),
    #[error("layout error: {0}")]
    Layout(String),
}
