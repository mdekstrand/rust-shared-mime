//! Support for loading XDG data files at runtime or for compilation.
mod dirs;
pub mod xdg_mimedef;
mod xdg_parse;

pub use dirs::xdg_mime_search_dirs;
pub use xdg_parse::parse_mime_package;
