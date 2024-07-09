use super::embedded_mime_db;

#[test]
fn test_json_subtype() {
    let db = embedded_mime_db();
    assert!(db.is_subtype("application/json", "text/plain"));
}

#[test]
fn test_text_subtype() {
    let db = embedded_mime_db();
    assert!(db.is_subtype("text/markdown", "text/plain"));
}

#[test]
fn test_unknown_text_subtype() {
    let db = embedded_mime_db();
    assert!(db.is_subtype("text/x-hackem-muche", "text/plain"));
}

#[test]
fn test_unknown_binary_subtype() {
    let db = embedded_mime_db();
    assert!(db.is_subtype("text/x-hackem-muche", "application/octet-stream"));
}

#[test]
fn test_img_binary_subtype() {
    let db = embedded_mime_db();
    assert!(db.is_subtype("image/png", "application/octet-stream"));
}

#[test]
fn test_text_binary_subtype() {
    let db = embedded_mime_db();
    assert!(db.is_subtype("text/plain", "application/octet-stream"));
}
