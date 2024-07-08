//! Types for shared-mime definitions parsed from package files.
//!
//! This uses quick-xml and Serde to deserialize the XML package definitions
//! from the [XDG Shared Mime Info database][xdg].
//!
//! [xdg]:
//!     https://specifications.freedesktop.org/shared-mime-info-spec/shared-mime-info-spec-latest.html
use serde::Deserialize;

use crate::record::{GlobRule, MimeTypeRecord};

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

    #[serde(rename = "$value")]
    pub elements: Vec<MimeTypeElement>,
}

/// Element in a MIME definition.
#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "kebab-case")]
pub enum MimeTypeElement {
    Comment(CommentElement),
    Glob(GlobElement),
    SubClassOf(TypeRefElement),
    Alias(TypeRefElement),
    Acronym(String),
    ExpandedAcronym(String),
    GenericIcon,
    Magic,
    Treemagic,
    #[serde(rename = "root-XML")]
    RootXML,
}

/// Comment (description) from the MIME database.
#[derive(Deserialize, Debug, Clone)]
pub struct CommentElement {
    #[serde(rename = "@lang")]
    pub lang: Option<String>,

    #[serde(rename = "$text")]
    pub value: String,
}

/// Glob element.
#[derive(Deserialize, Debug, Clone)]
pub struct GlobElement {
    #[serde(rename = "@pattern")]
    pub pattern: String,
    #[serde(rename = "@weight")]
    pub weight: Option<i32>,
    #[serde(rename = "@case-sensitive", default)]
    pub case_sensitive: bool,
}

/// Element that references another type.
#[derive(Deserialize, Debug, Clone)]
pub struct TypeRefElement {
    #[serde(rename = "@type")]
    pub mimetype: String,
}

impl MimeInfoPackage {
    pub fn into_records(self) -> Vec<MimeTypeRecord> {
        self.types.into_iter().map(|m| m.into()).collect()
    }
}

impl From<MimeType> for MimeTypeRecord {
    fn from(mime: MimeType) -> Self {
        return MimeTypeRecord {
            name: mime.name,
            description: mime
                .elements
                .iter()
                .filter_map(|e| match e {
                    MimeTypeElement::Comment(c) => Some(c),
                    _ => None,
                })
                .find(|c| match &c.lang {
                    None => true,
                    Some(lang) if lang == "en" => true,
                    _ => false,
                })
                .map(|c| c.value.clone()),
            globs: mime
                .elements
                .iter()
                .filter_map(|e| match e {
                    MimeTypeElement::Glob(g) => Some(g.into()),
                    _ => None,
                })
                .collect(),
            superclasses: mime
                .elements
                .iter()
                .filter_map(|e| match e {
                    MimeTypeElement::SubClassOf(tr) => Some(tr.mimetype.clone()),
                    _ => None,
                })
                .collect(),
            aliases: mime
                .elements
                .iter()
                .filter_map(|e| match e {
                    MimeTypeElement::Alias(tr) => Some(tr.mimetype.clone()),
                    _ => None,
                })
                .collect(),
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

impl From<&GlobElement> for GlobRule {
    fn from(glob: &GlobElement) -> Self {
        return GlobRule {
            pattern: glob.pattern.clone(),
            weight: glob.weight.unwrap_or(50),
            case_sensitive: glob.case_sensitive,
        };
    }
}
