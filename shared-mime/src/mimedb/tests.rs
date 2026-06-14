use {
    super::MimeDB,
    crate::record::{GlobRule, MimeTypeRecord},
};

// Weight should be considered when coalescing matches
#[test]
fn test_coalesce_weight() {
    let mut db = MimeDB::new();
    db.add_records(vec![MimeTypeRecord {
        name: "application/x-pagemaker".into(),
        description: None,
        globs: vec![GlobRule {
            pattern: "*.pmd".into(),
            weight: 50,
            case_sensitive: false,
        }],
        superclasses: vec![],
        aliases: vec![],
    }]);
    // x-piyopiyo should be the winner because it has highest weight
    db.add_records(vec![MimeTypeRecord {
        name: "audio/x-piyopiyo".into(),
        description: None,
        globs: vec![GlobRule {
            pattern: "*.pmd".into(),
            weight: 80,
            case_sensitive: false,
        }],
        superclasses: vec![],
        aliases: vec![],
    }]);
    db.add_records(vec![MimeTypeRecord {
        name: "application/x-whatever".into(),
        description: None,
        globs: vec![GlobRule {
            pattern: "*.pmd".into(),
            weight: 60,
            case_sensitive: false,
        }],
        superclasses: vec![],
        aliases: vec![],
    }]);
    assert_eq!(
        db.query_filename("test.pmd").best(),
        Some("audio/x-piyopiyo")
    );
}
