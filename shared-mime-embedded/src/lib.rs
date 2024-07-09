use data::EMBED_BYTES;
use postcard::from_bytes;

use shared_mime::{mimedb::MimeDB, record::MimeTypeRecord};

mod data;
#[cfg(test)]
mod tests;

pub fn embedded_mime_db() -> MimeDB {
    let mut db = MimeDB::new();
    let recs: Vec<MimeTypeRecord> = from_bytes(EMBED_BYTES).expect("embedded data decode failed");
    db.add_records(recs);
    db
}
