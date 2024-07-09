//! The [MimeDB] type for file type lookup.
use std::{cmp::Ordering, collections::HashMap};

mod build;
mod query;

use crate::{
    fnmatch::FileMatcher,
    search_queue::SearchQueue,
    strcache::{CachedString, StringCache},
};

/// Hold MIME data and facilitate  file type guessing.
pub struct MimeDB {
    names: StringCache,
    type_info: HashMap<CachedString, TypeInfo>,
    sequence: i32,
    globs: Vec<GlobRule>,
}

#[derive(Debug, Clone, Default)]
struct TypeInfo {
    description: Option<String>,
    aliases: Vec<CachedString>,
    parents: Vec<CachedString>,
}

#[derive(Debug, Clone)]
struct GlobRule {
    matcher: FileMatcher,
    sequence: i32,
    weight: i32,
    mimetype: String,
}

impl MimeDB {
    /// construct a new, empty MIME database.
    pub fn new() -> MimeDB {
        MimeDB {
            names: StringCache::new(),
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
        let mut queue: SearchQueue<CachedString> = SearchQueue::new();
        queue.maybe_add(self.names.cache(typ));
        while let Some(q) = queue.get() {
            if q == sup {
                return true;
            } else if sup == "text/plain" && q.starts_with("text/") {
                return true;
            }
            if let Some(info) = self.type_info.get(&q) {
                for pt in info.parents.iter() {
                    queue.maybe_add(pt.clone());
                }
            }
        }
        false
    }

    /// Get all known supertypes of the specified type (including itself).
    ///
    /// Types are in discovery order, so closer supertypes are at the beginning of the list.
    pub fn supertypes(&self, typ: &str) -> Vec<CachedString> {
        let mut types = Vec::new();
        let mut queue: SearchQueue<CachedString> = SearchQueue::new();
        let mut is_text = false;

        // start the queue with the search type
        queue.maybe_add(self.names.cache(typ));

        // pump until all types are done
        while let Some(qt) = queue.get() {
            // this is a supertype
            types.push(qt.clone());

            // is this a text type?
            if !is_text && qt.starts_with("text/") {
                is_text = true;
            }

            if let Some(info) = self.type_info.get(qt.as_ref()) {
                for st in info.parents.iter() {
                    queue.maybe_add(st.clone());
                }
            }
        }

        // add default parent relationships
        if is_text && !queue.saw("text/plain") {
            types.push(self.names.cache("text/plain"));
        }
        if !typ.starts_with("inode/") && !queue.saw("application/octet-stream") {
            types.push(self.names.cache("application/octet-stream"));
        }

        // types
        Vec::new()
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
