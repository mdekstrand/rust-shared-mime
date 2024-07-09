use std::borrow::Borrow;
use std::collections::{HashSet, VecDeque};
use std::hash::Hash;

/// A queue for graph searches; entries only get add once.
pub struct SearchQueue<T: Eq + Hash + Clone> {
    seen: HashSet<T>,
    queue: VecDeque<T>,
}

impl<T: Eq + Hash + Clone> SearchQueue<T> {
    pub fn new() -> SearchQueue<T> {
        SearchQueue {
            seen: HashSet::new(),
            queue: VecDeque::new(),
        }
    }

    pub fn maybe_add<I: Into<T>>(&mut self, item: I) -> bool {
        let item = item.into();
        if self.seen.insert(item.clone()) {
            self.queue.push_back(item);
            true
        } else {
            false
        }
    }

    pub fn get(&mut self) -> Option<T> {
        self.queue.pop_front()
    }

    #[allow(dead_code)]
    pub fn len(&self) -> usize {
        self.queue.len()
    }

    pub fn saw<Q: ?Sized>(&self, item: &Q) -> bool
    where
        T: Borrow<Q>,
        Q: Hash + Eq,
    {
        self.seen.contains(item)
    }
}

#[test]
fn test_empty() {
    let mut queue = SearchQueue::<String>::new();
    assert!(queue.get().is_none())
}

#[test]
fn test_add_remove() {
    let mut queue = SearchQueue::<String>::new();
    queue.maybe_add("hello");
    assert!(queue.get() == Some("hello".into()));
    assert!(queue.get().is_none());
}

#[test]
fn test_add_twice() {
    let mut queue = SearchQueue::<String>::new();
    queue.maybe_add("hello");
    queue.maybe_add("bob");
    queue.maybe_add("hello");
    assert_eq!(queue.len(), 2);
    assert!(queue.get() == Some("hello".into()));
    assert!(queue.get() == Some("bob".into()));
    assert!(queue.get().is_none());
}
