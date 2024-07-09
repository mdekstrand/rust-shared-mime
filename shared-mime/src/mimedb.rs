//! The [MimeDB] type for file type lookup.
use std::collections::HashMap;

/// Hold MIME data and facilitate  file type guessing.
pub struct MimeDB {
    type_descriptions: HashMap<String, String>,
    extensions: HashMap<String, GlobTarget>,
    globs: HashMap<String, GlobTarget>,
}

#[derive(Debug, Clone)]
struct GlobTarget {
    weight: i32,
    types: Vec<String>,
}

impl Default for GlobTarget {
    fn default() -> Self {
        GlobTarget {
            weight: 50,
            types: Vec::new(),
        }
    }
}

impl MimeDB {
    fn new() -> Self {
        MimeDB {
            type_descriptions: HashMap::new(),
            extensions: HashMap::new(),
            globs: HashMap::new(),
        }
    }
}
