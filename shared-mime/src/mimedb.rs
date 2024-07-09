//! The [MimeDB] type for file type lookup.
use std::{
    cmp::Ordering,
    collections::{HashMap, HashSet},
    ffi::OsStr,
};

use log::*;

#[cfg(feature = "xdg-runtime")]
use crate::runtime::mimeinfo::SharedMimeInfo;
use crate::{answer::Answer, fnmatch::FileMatcher, record::MimeTypeRecord};

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

    pub fn add_records(&mut self, records: Vec<MimeTypeRecord>) {
        self.sequence += 1;
        for rec in records {
            let info = self.type_info.entry(rec.name.clone()).or_default();
            if let Some(desc) = rec.description {
                info.description = Some(desc);
            }
            info.aliases.extend(rec.aliases);
            info.parents.extend(rec.superclasses);
            for glob in rec.globs {
                let mut matcher = FileMatcher::new(glob.pattern);
                if glob.case_sensitive {
                    matcher = matcher.case_sensitive();
                }
                self.globs.push(GlobRule {
                    matcher,
                    sequence: self.sequence,
                    weight: glob.weight,
                    mimetype: rec.name.clone(),
                })
            }
        }
        self.globs.sort_by(|a, b| {
            // higher sequences (later packages) go first
            let seq = a.sequence.cmp(&b.sequence).reverse();
            // higher weights go first
            let weight = a.weight.cmp(&b.weight).reverse();
            seq.then(weight)
        });
    }

    #[cfg(feature = "xdg-runtime")]
    pub fn add_shared_mime_info(&mut self, info: SharedMimeInfo) {
        for dir in info.directories {
            debug!("adding MIME info from {}", dir.path.display());
            for pkg in dir.packages {
                self.add_records(pkg.types);
            }
        }
    }

    /// Look up MIME type information based only on a filename.
    pub fn match_filename<S: AsRef<OsStr>>(&self, path: S) -> Answer<'_> {
        let path = path.as_ref();
        debug!("looking up filename {}", path.to_string_lossy());
        let mut sw = None;
        let mut matches = Vec::new();
        let pbs = path.as_encoded_bytes();
        for glob in self.globs.iter() {
            if let Some((s, w)) = sw {
                if s > glob.sequence || w > glob.weight {
                    // done searching
                    break;
                }
            }
            if glob.matcher.matches(pbs) {
                sw = Some((glob.sequence, glob.weight));
                matches.push(glob.mimetype.as_str());
            }
        }
        let mut ambiguous = matches.len() > 1;
        // TODO: prefer matching literals
        // TODO: disambiguate by match length
        if ambiguous {
            // this is our own addition to the match logic
            // if we have multiple matches, but one is the supertype of the others, use it
            debug!(
                "{}: {} matches, sorting",
                path.to_string_lossy(),
                matches.len()
            );
            // put supertype first
            matches.sort_by(|a, b| self.compare_types(a, b).reverse());
            let root = matches[0];
            ambiguous = !matches[1..].iter().all(|t| self.is_subtype(t, root));
            if ambiguous {
                debug!("{}: ambiguous match", path.to_string_lossy())
            } else {
                debug!("{}: best match {}", path.to_string_lossy(), root)
            }
        }
        Answer::new(matches, ambiguous)
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
