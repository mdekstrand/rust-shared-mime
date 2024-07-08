//! Parse the XDG shared mime info set.
use std::fs;
use std::io;
use std::path::Path;

use quick_xml::de::from_reader;

use super::xdg_package::MimeInfoPackage;
use super::XDGError;

/// Parse a single package XML file from the shared mime database.
pub fn parse_mime_package(path: &Path) -> Result<MimeInfoPackage, XDGError> {
    let file = fs::File::open(path)?;
    let read = io::BufReader::new(file);
    let info: MimeInfoPackage = from_reader(read)?;
    Ok(info)
}
