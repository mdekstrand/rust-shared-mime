use data::EMBED_BYTES;
use log::*;
use postcard::from_bytes;

use shared_mime::record::MimeTypeRecord;
#[cfg(feature = "xdg-runtime")]
use shared_mime::runtime::load_xdg_mime_info;
use shared_mime::LoadError;
pub use shared_mime::{Answer, FileQuery, FileQueryBuilder, MimeDB};

mod data;
#[cfg(test)]
mod tests;

/// Get the embedded MIME info database.
pub fn embedded_mime_db() -> MimeDB {
    let mut db = MimeDB::new();
    let recs: Vec<MimeTypeRecord> = from_bytes(EMBED_BYTES).expect("embedded data decode failed");
    db.add_records(recs);
    debug!(
        "loaded embedded MIME info with {} types and {} globs",
        db.type_count(),
        db.glob_count()
    );
    db
}

/// Load the MIME info database.
///
/// This starts by loading the embedded database. If the `xdg-runtime` feature
/// is enabled, it then loads the XDG shared mime database installed on the
/// system, treating the embedded database as a directory of mime information
/// that preceeds any system information.
pub fn load_mime_db() -> Result<MimeDB, LoadError> {
    debug!("loading embedded MIME database");
    let mut db = embedded_mime_db();

    #[cfg(feature = "xdg-runtime")]
    {
        debug!("loading runtime MIME database");
        let nt = db.type_count();
        let ng = db.glob_count();
        match load_xdg_mime_info() {
            Ok(info) => db.add_shared_mime_info(info),
            Err(e) => warn!("error loading MIME info: {:?}", e),
        }
        debug!(
            "loaded shared MIME info with {} new types and {} new globs",
            db.type_count() - nt,
            db.glob_count() - ng
        );
    }

    Ok(db)
}
