/// A MIME type record from the shared mime database.
pub struct MimeTypeRecord {
    extensions: Vec<String>,
    globs: Vec<String>,
    superclasses: Vec<String>,
}

/// A glob rule in the database.
pub struct GlobRule {
    pattern: String,
    weight: i32,
    case_sensitive: bool,
}
