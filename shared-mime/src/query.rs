use std::{
    ffi::OsStr,
    fs::{self, Metadata},
    io::ErrorKind,
    path::Path,
};

use log::trace;

use crate::QueryError;

/// Information avaialble for a query to the database.
pub struct FileQuery<'a> {
    /// The file's name.
    pub(crate) filename: Option<&'a OsStr>,
    /// The file metadata.
    pub(crate) metadata: Option<Metadata>,
}

/// Builder for [FileQuery].
pub struct FileQueryBuilder<'name> {
    /// The file's name.
    filename: Option<&'name OsStr>,
    /// The file metadata.
    metadata: Option<Metadata>,
}

impl FileQuery<'_> {
    pub fn builder() -> FileQueryBuilder<'static> {
        FileQueryBuilder::new()
    }
}

impl<'name> FileQuery<'name> {
    pub fn for_filename(name: &'name OsStr) -> FileQuery<'name> {
        FileQuery::builder().filename(name).build()
    }

    pub fn for_path(path: &'name Path) -> Result<FileQuery, QueryError> {
        let mut fqb = Self::builder();

        if let Some(name) = path.file_name() {
            trace!("{}: using filename {:?}", path.display(), name);
            fqb = fqb.filename(name);
        }

        trace!("{}: looking up metadata", path.display());
        match fs::metadata(&path) {
            Ok(meta) => fqb = fqb.metadata(meta),
            Err(e) if e.kind() == ErrorKind::NotFound => {
                trace!("{}: file not found", path.display());
            }
            Err(e) => return Err(e.into()),
        };

        Ok(fqb.build())
    }
}

impl FileQueryBuilder<'static> {
    pub fn new() -> FileQueryBuilder<'static> {
        FileQueryBuilder {
            filename: None,
            metadata: None,
        }
    }
}

impl<'name> FileQueryBuilder<'name> {
    /// Build the file query.
    pub fn build(self) -> FileQuery<'name> {
        FileQuery {
            filename: self.filename,
            metadata: self.metadata,
        }
    }

    /// Set the filename for this builder.
    pub fn filename<'new>(self, name: &'new OsStr) -> FileQueryBuilder<'new> {
        FileQueryBuilder {
            filename: Some(name),
            ..self
        }
    }

    /// Set the metadata for this builder.
    pub fn metadata(self, meta: fs::Metadata) -> FileQueryBuilder<'name> {
        FileQueryBuilder {
            metadata: Some(meta),
            ..self
        }
    }
}
