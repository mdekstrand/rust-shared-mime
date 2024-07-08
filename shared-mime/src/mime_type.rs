/// A MIME type record from the shared mime database.
pub struct MimeTypeRecord {
    /// The full MIME type (type/subtype).
    pub name: String,
    /// The string description of this record.
    pub description: Option<String>,
    /// List of globs (with priorities) for the record.
    pub globs: Vec<GlobRule>,
    /// List of this record's immediate superclasses.
    pub superclasses: Vec<String>,
    /// Aliases for this record.
    pub aliases: Vec<String>,
}

/// A glob rule in the database.
pub struct GlobRule {
    // Glob pattern.
    pub pattern: String,
    // Glob weight.
    pub weight: i32,
    // Whether this rule is case-sensitive.
    pub case_sensitive: bool,
}
