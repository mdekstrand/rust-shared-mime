use std::{ffi::OsStr, fs::Metadata};

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
}
