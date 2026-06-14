//! MIME DB querying implementation.
use std::ffi::OsStr;
use std::fs::Metadata;
#[cfg(unix)]
use std::os::unix::fs::FileTypeExt;
use std::os::unix::fs::MetadataExt;

use log::*;

use crate::{query::FileQuery, Answer, QueryError};

use super::MimeDB;

enum MetaAnswer {
    Inode(&'static str),
    File(u64),
}

fn type_for_meta(meta: &Metadata) -> MetaAnswer {
    let ft = meta.file_type();
    if ft.is_dir() {
        return MetaAnswer::Inode("inode/directory");
    } else if ft.is_symlink() {
        return MetaAnswer::Inode("inode/symlink");
    }

    #[cfg(unix)]
    if ft.is_block_device() {
        return MetaAnswer::Inode("inode/blockdevice");
    } else if ft.is_char_device() {
        return MetaAnswer::Inode("inode/chardevice");
    } else if ft.is_fifo() {
        return MetaAnswer::Inode("inode/fifo");
    } else if ft.is_socket() {
        return MetaAnswer::Inode("inode/socket");
    }

    MetaAnswer::File(meta.size())
}

#[derive(Clone, Copy, Debug)]
struct Match<'a> {
    mime: &'a str,
    weight: i32,
}

impl MimeDB {
    /// Query the MIME database.
    pub fn query(&self, query: &FileQuery<'_>) -> Result<Answer<'_>, QueryError> {
        let dbg_name = if let Some(name) = query.filename {
            name.to_string_lossy()
        } else {
            "⟨unnamed⟩".into()
        };
        // first step: check for special files, if we have metadata
        let size = if let Some(meta) = &query.metadata {
            debug!("{}: looking up with metadata", dbg_name);
            match type_for_meta(meta) {
                // if we have a special title, all done!
                MetaAnswer::Inode(tstr) => return Ok(Answer::definite(tstr)),
                MetaAnswer::File(size) => Some(size),
            }
        } else {
            None
        };

        // next step: look up based on filename
        let mut ans = Answer::unknown();
        if let Some(name) = query.filename {
            debug!("{}: looking up with file name", dbg_name);
            ans = self.query_filename(name);
        }

        if ans.is_unknown() && size == Some(0) {
            ans = Answer::definite("application/x-zerosize")
        }

        // TODO: detect text files
        if ans.is_unknown() {
            ans = Answer::definite("application/octet-stream")
        }

        Ok(ans)
    }

    /// Use metadata to detect file types.
    ///
    /// This function can only detect the `inode/` types and `application/octet-stream`.
    pub fn query_meta(&self, meta: &Metadata) -> Answer<'_> {
        match type_for_meta(meta) {
            MetaAnswer::Inode(mt) => Answer::definite(mt),
            MetaAnswer::File(_) => Answer::definite("application/octet-stream"),
        }
    }

    /// Look up MIME type information based only on a filename.
    pub fn query_filename<S: AsRef<OsStr>>(&self, name: S) -> Answer<'_> {
        let name = name.as_ref();
        let display = name.to_string_lossy();
        debug!("looking up filename {}", display);
        let mut matches = Vec::new();
        let pbs = name.as_encoded_bytes();
        for glob in self.globs.iter() {
            if glob.matcher.matches(pbs) {
                matches.push(Match {
                    mime: glob.mimetype.as_str(),
                    weight: glob.weight,
                });
            }
        }
        let ambiguous = self.coalesce_fn_matches(&display, &mut matches);
        Answer::new(matches.into_iter().map(|m| m.mime).collect(), ambiguous)
    }

    fn coalesce_fn_matches(&self, name: &str, matches: &mut Vec<Match>) -> bool {
        let mut ambiguous = matches.len() > 1;
        // TODO: prefer matching literals
        // TODO: disambiguate by match length
        if ambiguous {
            // this is our own addition to the match logic
            // if we have multiple matches, but one is the supertype of the others, use it
            debug!("{}: {} matches, sorting", name, matches.len());
            // put supertype first
            matches.sort_by(|a, b| {
                self.compare_types(a.mime, b.mime)
                    .reverse()
                    .then_with(|| a.weight.cmp(&b.weight).reverse())
            });
            let root = matches[0];
            ambiguous = !matches[1..]
                .iter()
                .all(|m| self.is_subtype(m.mime, root.mime));
            if matches[1..].iter().any(|m| root.weight > m.weight) {
                ambiguous = false;
            }
            if ambiguous {
                debug!("{}: ambiguous match", name)
            } else {
                debug!("{}: best match {}", name, root.mime)
            }
        }
        ambiguous
    }
}
