use std::ffi::OsString;

use crate::embedded_mime_db;

#[test]
fn test_text_file() {
    let db = embedded_mime_db();
    let answer = db.match_filename(OsString::from("foo.txt"));
    assert_eq!(answer.best(), Some("text/plain"));
}

#[test]
fn test_png_image() {
    let db = embedded_mime_db();
    let answer = db.match_filename(OsString::from("foo.png"));
    assert_eq!(answer.best(), Some("image/png"));
}

#[test]
fn test_json_file() {
    // due to our custom logic, JSON should resolve correctly
    let db = embedded_mime_db();
    let answer = db.match_filename(OsString::from("foo.json"));
    assert_eq!(answer.best(), Some("application/json"));
    assert!(answer.all_types().len() > 1);
}
