//! Pin-based string cache.

use std::fmt::{Debug, Display};
use std::{borrow::Borrow, cell::RefCell, collections::HashSet, hash::Hash, ops::Deref, rc::Rc};

/// Cache to reduce duplicated strings in memory.
pub struct StringCache {
    cache: RefCell<HashSet<CachedString>>,
}

impl StringCache {
    pub fn new() -> StringCache {
        StringCache {
            cache: RefCell::new(HashSet::new()),
        }
    }

    pub fn cache<S: AsRef<str>>(&self, string: S) -> CachedString {
        // slightly inefficient to search up to 3 times, but keeps borrow checker happy
        let cache = self.cache.borrow();
        if let Some(cached) = cache.get(string.as_ref()).clone() {
            return cached.clone();
        }
        drop(cache);

        let cached = CachedString::create(string.as_ref().to_string());
        let mut mcache = self.cache.borrow_mut();
        mcache.insert(cached.clone());
        cached
    }
}

/// A cached string.
///
/// This string contains a reference to the underlying shared cached string, and
/// can be cloned cheaply (it's just an [Rc]).
#[derive(Clone)]
pub struct CachedString {
    string: Rc<String>,
}

impl CachedString {
    fn create(string: String) -> CachedString {
        CachedString {
            string: Rc::new(string),
        }
    }

    fn as_str(&self) -> &str {
        &*self.string
    }
}

impl Debug for CachedString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(self.string.as_ref(), f)
    }
}
impl Display for CachedString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Display::fmt(self.string.as_ref(), f)
    }
}

impl Deref for CachedString {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        self.as_str()
    }
}

impl AsRef<str> for CachedString {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl Borrow<str> for CachedString {
    fn borrow(&self) -> &str {
        self.as_str()
    }
}

impl PartialEq for CachedString {
    fn eq(&self, other: &Self) -> bool {
        self.as_str() == other.as_str()
    }
}

impl PartialEq<str> for CachedString {
    fn eq(&self, other: &str) -> bool {
        self.as_str() == other
    }
}

impl PartialEq<&str> for CachedString {
    fn eq(&self, other: &&str) -> bool {
        self.as_str() == *other
    }
}

impl Eq for CachedString {}

impl Hash for CachedString {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.as_str().hash(state)
    }
}
