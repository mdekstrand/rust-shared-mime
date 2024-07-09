//! MIME DB querying implementation.
use std::ffi::OsStr;

use log::*;

use crate::Answer;

use super::MimeDB;

impl MimeDB {
    /// Look up MIME type information based only on a filename.
    pub fn query_filename<S: AsRef<OsStr>>(&self, name: S) -> Answer<'_> {
        let name = name.as_ref();
        let display = name.to_string_lossy();
        debug!("looking up filename {}", display);
        let mut sw = None;
        let mut matches = Vec::new();
        let pbs = name.as_encoded_bytes();
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
        let ambiguous = self.coalesce_fn_matches(&display, &mut matches);
        Answer::new(matches, ambiguous)
    }

    fn coalesce_fn_matches(&self, name: &str, matches: &mut Vec<&str>) -> bool {
        let mut ambiguous = matches.len() > 1;
        // TODO: prefer matching literals
        // TODO: disambiguate by match length
        if ambiguous {
            // this is our own addition to the match logic
            // if we have multiple matches, but one is the supertype of the others, use it
            debug!("{}: {} matches, sorting", name, matches.len());
            // put supertype first
            matches.sort_by(|a, b| self.compare_types(a, b).reverse());
            let root = matches[0];
            ambiguous = !matches[1..].iter().all(|t| self.is_subtype(t, root));
            if ambiguous {
                debug!("{}: ambiguous match", name)
            } else {
                debug!("{}: best match {}", name, root)
            }
        }
        ambiguous
    }
}
