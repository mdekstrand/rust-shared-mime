/// Result of looking up a MIME type.
#[derive(Debug, Clone)]
pub struct Answer<'a> {
    types: Vec<&'a str>,
    ambiguous: bool,
}

impl<'a> Answer<'a> {
    pub(crate) fn new(types: Vec<&'a str>, ambiguous: bool) -> Answer<'a> {
        Answer { types, ambiguous }
    }

    /// Query whether this answer is definite (resolved to a single, known type).
    pub fn is_definite(&self) -> bool {
        self.types.len() >= 1 && !self.ambiguous
    }

    /// Query whether this answer is unknown (no resulting types).
    pub fn is_unknown(&self) -> bool {
        self.types.len() == 0
    }

    /// Query whether this answer is ambiguous (multiple matching types).
    pub fn is_ambiguous(&self) -> bool {
        self.ambiguous
    }

    /// Get the best type, if known.  Returns [None] when no type is found or the type is ambiguous.
    pub fn best(&self) -> Option<&'a str> {
        if self.ambiguous || self.types.is_empty() {
            None
        } else {
            Some(self.types[0])
        }
    }

    /// Get all matching types.
    pub fn all_types(&self) -> &'_ [&'a str] {
        &self.types
    }
}
