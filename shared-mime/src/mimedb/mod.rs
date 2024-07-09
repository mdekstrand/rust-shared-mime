//! The [MimeDB] type for file type lookup.
use std::{
    cmp::Ordering,
    collections::{HashMap, HashSet},
};

mod build;
mod query;

use crate::fnmatch::FileMatcher;

/// Hold MIME data and facilitate  file type guessing.
pub struct MimeDB {
    type_info: HashMap<String, TypeInfo>,
    sequence: i32,
    globs: Vec<GlobRule>,
}

#[derive(Debug, Clone, Default)]
struct TypeInfo {
    description: Option<String>,
    aliases: Vec<String>,
    parents: Vec<String>,
}

#[derive(Debug, Clone)]
struct GlobRule {
    matcher: FileMatcher,
    sequence: i32,
    weight: i32,
    mimetype: String,
}

impl MimeDB {
    pub fn new() -> MimeDB {
        MimeDB {
            type_info: HashMap::new(),
            sequence: 0,
            globs: Vec::new(),
        }
    }

    /// Get the number of known types.
    pub fn type_count(&self) -> usize {
        self.type_info.len()
    }

    /// Get the number of globs.
    pub fn glob_count(&self) -> usize {
        self.globs.len()
    }

    /// Query whether one type is a subtype of another.
    pub fn is_subtype(&self, typ: &str, sup: &str) -> bool {
        // everything is an octet stream
        if sup == "application/octet-stream" && !typ.starts_with("inode/") {
            return true;
        }
        let mut seen = HashSet::new();
        let mut types = vec![typ];
        while let Some(q) = types.pop() {
            if q == sup {
                return true;
            } else if sup == "text/plain" && q.starts_with("text/") {
                return true;
            }
            if let Some(info) = self.type_info.get(q) {
                for pt in info.parents.iter() {
                    if !seen.contains(pt) {
                        seen.insert(pt);
                        // we can stack, it's fine
                        types.push(pt);
                    }
                }
            }
        }
        false
    }

    /// Order two types, where a type is less than its supertypes
    pub fn compare_types(&self, a: &str, b: &str) -> Ordering {
        if self.is_subtype(a, b) {
            Ordering::Less
        } else if self.is_subtype(b, a) {
            Ordering::Greater
        } else {
            Ordering::Equal
        }
    }
}
