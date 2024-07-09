//! MIME DB querying implementation.
use std::ffi::OsStr;

use log::*;

use crate::Answer;

use super::MimeDB;

impl MimeDB {
    /// Look up MIME type information based only on a filename.
    pub fn query_filename<S: AsRef<OsStr>>(&self, path: S) -> Answer<'_> {
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
}
