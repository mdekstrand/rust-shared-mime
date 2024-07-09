//! Glob-based filename matching utility.
//!
//! This is like `fnmatch`, matching glob patterns. It does not treat '/'
//! specially, as it is just intended for final file names.

use std::mem::replace;

/// Match files against a pattern.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct FileMatcher {
    pub rule: MatchRule,
    pub case_sensitive: bool,
}

/// A match rule for matching files.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum MatchRule {
    Literal(Vec<u8>),
    Suffix(Vec<u8>),
    Pattern(Vec<MatchElement>),
}

/// Single elements of a pattern.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum MatchElement {
    Star,
    Wildcard,
    Range(u8, u8),
    Literal(Vec<u8>),
}

impl FileMatcher {
    pub fn new<S: AsRef<str>>(pattern: S) -> FileMatcher {
        FileMatcher {
            rule: pattern.as_ref().into(),
            case_sensitive: false,
        }
    }

    pub fn case_sensitive(self) -> FileMatcher {
        FileMatcher {
            case_sensitive: true,
            ..self
        }
    }

    pub fn matches(&self, path: &[u8]) -> bool {
        if self.case_sensitive {
            self.rule.matches_with_case(path)
        } else {
            self.rule.matches(path)
        }
    }
}

impl MatchRule {
    pub fn matches(&self, path: &[u8]) -> bool {
        match self {
            MatchRule::Literal(lit) => lit.eq_ignore_ascii_case(path),
            MatchRule::Suffix(sfx) => {
                sfx.eq_ignore_ascii_case(&path[path.len().saturating_sub(sfx.len())..])
            }
            MatchRule::Pattern(pat) => seq_matches(pat, path),
        }
    }

    pub fn matches_with_case(&self, path: &[u8]) -> bool {
        match self {
            MatchRule::Literal(lit) => lit == path,
            MatchRule::Suffix(sfx) => path.ends_with(&sfx),
            MatchRule::Pattern(pat) => seq_matches_with_case(pat, path),
        }
    }
}

impl From<&str> for MatchRule {
    fn from(value: &str) -> Self {
        let elts = parse_pattern(value.as_bytes());
        match &elts[..] {
            [MatchElement::Literal(lit)] => MatchRule::Literal(lit.clone()),
            [MatchElement::Star, MatchElement::Literal(sfx)] => MatchRule::Suffix(sfx.clone()),
            _ => MatchRule::Pattern(elts),
        }
    }
}

/// Match a sequence of rules against a string.
///
/// TODO: This is inefficient and recursive, can cause problems on very long
/// paths and pathological patterns.
fn seq_matches(pat: &[MatchElement], path: &[u8]) -> bool {
    match &pat[..] {
        [] => path.is_empty(),
        _ if pat.is_empty() => false,
        [MatchElement::Literal(lit)] => lit.eq_ignore_ascii_case(path),
        [MatchElement::Literal(lit), ..] => {
            path.len() >= lit.len()
                && lit.eq_ignore_ascii_case(&path[0..lit.len()])
                && seq_matches(&pat[1..], &path[lit.len()..])
        }
        [MatchElement::Star] => true,
        [MatchElement::Star, ..] => {
            for i in 0..path.len() {
                if seq_matches(&pat[1..], &path[i..]) {
                    return true;
                }
            }
            false
        }
        [MatchElement::Wildcard, ..] => path.len() >= 1 && seq_matches(&pat[1..], &path[1..]),
        [MatchElement::Range(s, e), ..] => {
            path.len() >= 1
                && path[0].to_ascii_lowercase() >= s.to_ascii_lowercase()
                && path[0].to_ascii_lowercase() <= e.to_ascii_lowercase()
                && seq_matches(&pat[1..], &path[1..])
        }
    }
}

fn seq_matches_with_case(pat: &[MatchElement], path: &[u8]) -> bool {
    match &pat[..] {
        [] => path.is_empty(),
        _ if pat.is_empty() => false,
        [MatchElement::Literal(lit)] => lit == path,
        [MatchElement::Literal(lit), ..] => {
            path.len() >= lit.len()
                && lit == &path[0..lit.len()]
                && seq_matches_with_case(&pat[1..], &path[lit.len()..])
        }
        [MatchElement::Star] => true,
        [MatchElement::Star, ..] => {
            for i in 0..path.len() {
                if seq_matches_with_case(&pat[1..], &path[i..]) {
                    return true;
                }
            }
            false
        }
        [MatchElement::Wildcard, ..] => {
            path.len() >= 1 && seq_matches_with_case(&pat[1..], &path[1..])
        }
        [MatchElement::Range(s, e), ..] => {
            path.len() >= 1
                && path[0] >= *s
                && path[0] <= *e
                && seq_matches_with_case(&pat[1..], &path[1..])
        }
    }
}

fn parse_pattern(pattern: &[u8]) -> Vec<MatchElement> {
    let n = pattern.len();
    let mut elts = Vec::new();
    let mut current = Vec::with_capacity(n);
    let mut pos = 0;
    while pos < n {
        match pattern[pos] {
            b'*' => {
                maybe_push_literal(&mut elts, &mut current, n - pos);
                elts.push(MatchElement::Star)
            }
            b'?' => {
                maybe_push_literal(&mut elts, &mut current, n - pos);
                elts.push(MatchElement::Wildcard)
            }
            b'[' if n > pos + 4 && pattern[pos + 2] == b'-' && pattern[pos + 4] == b']' => {
                maybe_push_literal(&mut elts, &mut current, n - pos);
                let start = pattern[pos + 1];
                let end = pattern[pos + 3];
                pos += 4;
                elts.push(MatchElement::Range(start, end));
            }
            c => current.push(c),
        }
        pos += 1;
    }
    maybe_push_literal(&mut elts, &mut current, n - pos);
    elts
}

fn maybe_push_literal(elts: &mut Vec<MatchElement>, current: &mut Vec<u8>, n: usize) {
    if current.len() > 0 {
        elts.push(MatchElement::Literal(replace(
            current,
            Vec::with_capacity(n),
        )));
    }
}

#[test]
fn test_parse_literal() {
    let pat = parse_pattern(b"duam.xnaht");
    assert_eq!(&pat, &[MatchElement::Literal(b"duam.xnaht".into())])
}

#[test]
fn test_parse_suffix() {
    let pat = parse_pattern(b"*.fooels");
    assert_eq!(
        pat,
        &[MatchElement::Star, MatchElement::Literal(b".fooels".into())]
    )
}

#[test]
fn test_parse_prefix() {
    let pat = parse_pattern(b"eldib.*");
    assert_eq!(
        pat,
        &[MatchElement::Literal("eldib.".into()), MatchElement::Star]
    )
}

#[test]
fn test_parse_star_middle() {
    let pat = parse_pattern(b"xixaxa.*.xuxaxa");
    assert_eq!(
        pat,
        &[
            MatchElement::Literal("xixaxa.".into()),
            MatchElement::Star,
            MatchElement::Literal(".xuxaxa".into())
        ]
    )
}

#[test]
fn test_parse_wildcard() {
    let pat = parse_pattern(b"man.?");
    assert_eq!(
        pat,
        &[MatchElement::Literal("man.".into()), MatchElement::Wildcard,]
    )
}

#[test]
fn test_parse_star_wildcard() {
    let pat = parse_pattern(b"*.?");
    assert_eq!(
        pat,
        &[
            MatchElement::Star,
            MatchElement::Literal(".".into()),
            MatchElement::Wildcard,
        ]
    )
}

#[test]
fn test_parse_range() {
    let pat = parse_pattern(b"*.so.[0-9]");
    assert_eq!(
        pat,
        &[
            MatchElement::Star,
            MatchElement::Literal(".so.".into()),
            MatchElement::Range(b'0', b'9'),
        ]
    )
}

#[test]
fn test_parse_range_not_end() {
    let pat = parse_pattern(b"*.so.[0-9].gz");
    assert_eq!(
        pat,
        &[
            MatchElement::Star,
            MatchElement::Literal(".so.".into()),
            MatchElement::Range(b'0', b'9'),
            MatchElement::Literal(".gz".into()),
        ]
    )
}

#[test]
fn test_basic_literal() {
    let pat = FileMatcher::new("hackem.muche");
    assert_eq!(pat.rule, MatchRule::Literal(b"hackem.muche".into()));
    assert!(pat.matches(b"hackem.muche"));
    assert!(pat.matches(b"HACKEM.muche"));
    assert!(!pat.matches(b"foobie.muche"))
}

#[test]
fn test_basic_lit_cs() {
    let pat = FileMatcher::new("hackem.muche").case_sensitive();
    assert!(pat.matches(b"hackem.muche"));
    assert!(!pat.matches(b"HACKEM.muche"));
    assert!(!pat.matches(b"foobie.muche"))
}

#[test]
fn test_basic_star_suffix() {
    let pat = FileMatcher::new("*.muche");
    assert_eq!(pat.rule, MatchRule::Suffix(b".muche".into()));
    assert!(pat.matches(b"hackem.muche"));
    assert!(pat.matches(b"HACKEM.muche"));
    assert!(pat.matches(b"foobie.MUCHE"));
    assert!(!pat.matches(b"foobie.bletch"))
}

#[test]
fn test_basic_star_suffix_cs() {
    let pat = FileMatcher::new("*.muche").case_sensitive();
    assert!(pat.matches(b"hackem.muche"));
    assert!(pat.matches(b"HACKEM.muche"));
    assert!(!pat.matches(b"hackem.MuChe"));
    assert!(!pat.matches(b"foobie.bletch"))
}

#[test]
fn test_basic_star_prefix() {
    let pat = FileMatcher::new("foobie.*");
    assert!(pat.matches(b"foobie.muche"));
    assert!(pat.matches(b"FOOBIE.muche"));
    assert!(pat.matches(b"FOOBIE."));
    assert!(!pat.matches(b"hackem.bletch"))
}

#[test]
fn test_basic_star_prefix_cs() {
    let pat = FileMatcher::new("foobie.*").case_sensitive();
    assert!(pat.matches(b"foobie.muche"));
    assert!(pat.matches(b"foobie."));
    assert!(!pat.matches(b"FOOBIE.muche"));
    assert!(!pat.matches(b"hackem.bletch"))
}

#[test]
fn test_star_middle() {
    let pat = FileMatcher::new("part-*.sh");
    assert!(pat.matches(b"part-fish.sh"));
    assert!(!pat.matches(b"piece-fish.sh"));
}

#[test]
fn test_class_match() {
    let pat = FileMatcher::new("*.so.[0-9]").case_sensitive();
    assert!(pat.matches(b"libc.so.6"));
    assert!(!pat.matches(b"libc.so.X"));
    assert!(!pat.matches(b"libc.sq.7"));
}
