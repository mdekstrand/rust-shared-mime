//! Parse the XDG shared mime info set.
use std::fs;
use std::io;
use std::path::Path;

use quick_xml::de::{from_reader, DeError};
use thiserror::Error;

use super::xdg_mimedef::MimeInfo;

/// Error type for mime-info parse failures.
#[derive(Error, Debug)]
pub enum ParseError {
    #[error("I/O error: {0}")]
    IO(#[from] io::Error),
    #[error("XML deserialize error: {0}")]
    Deserialize(#[from] DeError),
}

/// Parse a package XML file from the shared mime database.
pub fn parse_mime_package(path: &Path) -> Result<MimeInfo, ParseError> {
    let file = fs::File::open(path)?;
    let read = io::BufReader::new(file);
    let info: MimeInfo = from_reader(read)?;
    Ok(info)
}
