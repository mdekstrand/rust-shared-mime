//! Types for shared-mime definitions parsed from package files.
//!
//! This uses quick-xml and Serde to deserialize the XML package definitions
//! from the [XDG Shared Mime Info database][xdg].
//!
//! [xdg]:
//!     https://specifications.freedesktop.org/shared-mime-info-spec/shared-mime-info-spec-latest.html
use serde::Deserialize;

// Shared mime info database.
#[derive(Deserialize, Debug, Clone)]
pub struct MimeInfo {
    #[serde(rename = "mime-type")]
    pub types: Vec<MimeType>,
}

/// Single MIME type record from the database.
#[derive(Deserialize, Debug, Clone)]
pub struct MimeType {
    #[serde(rename = "@type")]
    pub name: String,

    #[serde(rename = "comment", default)]
    pub comments: Vec<Comment>,
    #[serde(rename = "glob", default)]
    pub globs: Vec<Glob>,
    #[serde(rename = "sub-class-of", default)]
    pub superclasses: Vec<String>,

    pub acronym: Option<String>,
    #[serde(rename = "expanded-acronym")]
    pub expanded_acronym: Option<String>,
}

/// Comment (description) from the MIME database.
#[derive(Deserialize, Debug, Clone)]
pub struct Comment {
    #[serde(rename = "@lang")]
    pub lang: Option<String>,

    #[serde(rename = "$text")]
    pub value: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Glob {
    #[serde(rename = "@pattern")]
    pub pattern: String,
    #[serde(rename = "@weight", default = "default_weight")]
    pub weight: i32,
    #[serde(rename = "@case-sensitive", default)]
    pub case_sensitive: bool,
}

fn default_weight() -> i32 {
    return 50;
}
