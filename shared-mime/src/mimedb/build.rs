use log::*;

use super::{GlobRule, MimeDB};
#[cfg(feature = "xdg-runtime")]
use crate::runtime::mimeinfo::SharedMimeInfo;
use crate::{fnmatch::FileMatcher, record::MimeTypeRecord};

impl MimeDB {
    pub fn add_records(&mut self, records: Vec<MimeTypeRecord>) {
        self.sequence += 1;
        for rec in records {
            let name = self.names.cache(&rec.name);
            let info = self.type_info.entry(name).or_default();
            if let Some(desc) = rec.description {
                info.description = Some(desc);
            }
            info.aliases
                .extend(rec.aliases.into_iter().map(|c| self.names.cache(c)));
            info.parents
                .extend(rec.superclasses.into_iter().map(|c| self.names.cache(c)));
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
}
