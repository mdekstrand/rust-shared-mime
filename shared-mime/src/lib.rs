//! Support for parsing entries from the XDG Shared Mime Info database.
pub mod answer;
pub mod error;
mod fnmatch;
pub mod mimedb;
pub mod query;
pub mod record;
#[cfg(feature = "xdg-runtime")]
pub mod runtime;
mod search_queue;
mod strcache;

use log::*;

pub use error::{LoadError, QueryError};

pub use answer::Answer;
pub use mimedb::MimeDB;
pub use query::{FileQuery, FileQueryBuilder};

/// Load the MIME database.
#[cfg(not(feature = "xdg-runtime"))]
pub fn load_mime_db() -> Result<MimeDB, LoadError> {
    Err(LoadError::Unavailable)
}

/// Load the MIME database.
#[cfg(feature = "xdg-runtime")]
pub fn load_mime_db() -> Result<MimeDB, LoadError> {
    use runtime::load_xdg_mime_info;

    let mut db = MimeDB::new();
    let info = load_xdg_mime_info()?;
    db.add_shared_mime_info(info);
    debug!(
        "loaded shared MIME info with {} types and {} globs",
        db.type_count(),
        db.glob_count()
    );
    Ok(db)
}
