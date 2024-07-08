//! XDG Shared MIME Info database representation.

use std::{
    fs::read_dir,
    path::{Path, PathBuf},
};

use log::*;
use serde::Serialize;

use crate::record::MimeTypeRecord;

use super::{parse_mime_package, xdg_mime_search_dirs, XDGError};

/// Container for the fully-parsed Shared Mime Info across search directories.
#[derive(Serialize, Debug, Clone)]
pub struct SharedMimeInfo {
    /// The XDG mime directories in processing order.
    pub directories: Vec<SMIDir>,
}

/// A single directory in the shared mime database.
#[derive(Serialize, Debug, Clone)]
pub struct SMIDir {
    pub path: PathBuf,
    pub packages: Vec<SMIPackage>,
}

/// A single package within the shared mime database.
#[derive(Serialize, Debug, Clone)]
pub struct SMIPackage {
    pub filename: String,
    pub types: Vec<MimeTypeRecord>,
}

/// Load the full XDG mime info database from all available files.
pub fn load_xdg_mime_info() -> Result<SharedMimeInfo, XDGError> {
    let dirs = xdg_mime_search_dirs();
    Ok(SharedMimeInfo {
        directories: dirs
            .into_iter()
            .filter_map(|d| load_xdg_mime_dir(d).transpose())
            .collect::<Result<Vec<_>, _>>()?,
    })
}

fn load_xdg_mime_dir<P: AsRef<Path>>(path: P) -> Result<Option<SMIDir>, XDGError> {
    let path = path.as_ref();
    let mut buf = path.to_path_buf();
    buf.push("packages");
    debug!("looking for packages in {}", path.display());
    if !buf.try_exists()? {
        debug!("{} does not exist", buf.display());
        return Ok(None);
    }

    let mut packages = Vec::new();
    for entry in read_dir(&buf)? {
        let entry = entry?;
        let ep = entry.path();
        let ext = ep.extension().map_or("".into(), |e| e.to_string_lossy());
        debug!("extension {}", ext);
        if entry.file_name().as_encoded_bytes()[0] != b'.' && ext == "xml" {
            debug!("reading package file {}", ep.display());
            packages.push(SMIPackage {
                filename: ep
                    .file_name()
                    .ok_or(XDGError::Layout("package missing filename".into()))?
                    .to_string_lossy()
                    .to_string(),
                types: parse_mime_package(&ep)?
                    .types
                    .into_iter()
                    .map(MimeTypeRecord::from)
                    .collect(),
            })
        } else {
            debug!("ignoring file {}", ep.display());
        }
    }

    Ok(Some(SMIDir {
        path: path.to_path_buf(),
        packages,
    }))
}
