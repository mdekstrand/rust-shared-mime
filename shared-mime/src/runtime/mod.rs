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
pub mod xdg_package;
mod xdg_parse;

pub use dirs::xdg_mime_search_dirs;
pub use xdg_parse::parse_mime_package;
