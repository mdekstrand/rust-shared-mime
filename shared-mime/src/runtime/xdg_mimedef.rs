//! Types for shared-mime definitions parsed from package files.
//!
//! This uses quick-xml and Serde to deserialize the XML package definitions
//! from the [XDG Shared Mime Info database][xdg].
//!
//! [xdg]:
//!     https://specifications.freedesktop.org/shared-mime-info-spec/shared-mime-info-spec-latest.html
use serde::Deserialize;

use crate::mime_type::{GlobRule, MimeTypeRecord};

// Shared mime info database.
#[derive(Deserialize, Debug, Clone)]
pub struct MimeInfoPackage {
    #[serde(rename = "mime-type")]
    pub types: Vec<MimeType>,
}

/// Single MIME type record from the database.
#[derive(Deserialize, Debug, Clone)]
pub struct MimeType {
    #[serde(rename = "@type")]
    pub name: String,

    #[serde(rename = "comment", default)]
    pub comments: Vec<CommentElement>,
    #[serde(rename = "glob", default)]
    pub globs: Vec<GlobElement>,
    #[serde(rename = "sub-class-of", default)]
    pub superclasses: Vec<String>,
    #[serde(rename = "alias", default)]
    pub aliases: Vec<AliasElement>,

    pub acronym: Option<String>,
    #[serde(rename = "expanded-acronym")]
    pub expanded_acronym: Option<String>,
}

/// Comment (description) from the MIME database.
#[derive(Deserialize, Debug, Clone)]
pub struct CommentElement {
    #[serde(rename = "@lang")]
    pub lang: Option<String>,

    #[serde(rename = "$text")]
    pub value: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct GlobElement {
    #[serde(rename = "@pattern")]
    pub pattern: String,
    #[serde(rename = "@weight")]
    pub weight: Option<i32>,
    #[serde(rename = "@case-sensitive", default)]
    pub case_sensitive: bool,
}

#[derive(Deserialize, Debug, Clone)]
pub struct AliasElement {
    #[serde(rename = "type")]
    pub mimetype: String,
}

impl From<MimeType> for MimeTypeRecord {
    fn from(mime: MimeType) -> Self {
        let desc = mime.comments.into_iter().find(|c| match &c.lang {
            None => true,
            Some(lang) if lang == "en" => true,
            _ => false,
        });
        return MimeTypeRecord {
            name: mime.name,
            description: desc.map(|c| c.value),
            globs: mime.globs.into_iter().map(|g| g.into()).collect(),
            superclasses: mime.superclasses,
            aliases: mime.aliases.into_iter().map(|a| a.mimetype).collect(),
        };
    }
}

impl From<GlobElement> for GlobRule {
    fn from(glob: GlobElement) -> Self {
        return GlobRule {
            pattern: glob.pattern,
            weight: glob.weight.unwrap_or(50),
            case_sensitive: glob.case_sensitive,
        };
    }
}
