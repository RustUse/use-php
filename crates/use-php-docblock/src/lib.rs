#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

use core::{fmt, str::FromStr};
use std::error::Error;

macro_rules! doc_text_newtype {
    ($name:ident) => {
        #[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
        pub struct $name(String);

        impl $name {
            pub fn new(input: &str) -> Result<Self, PhpDocblockError> {
                let trimmed = input.trim();
                if trimmed.is_empty() {
                    Err(PhpDocblockError::Empty)
                } else {
                    Ok(Self(trimmed.to_string()))
                }
            }

            pub fn as_str(&self) -> &str {
                &self.0
            }
        }

        impl fmt::Display for $name {
            fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                formatter.write_str(self.as_str())
            }
        }

        impl FromStr for $name {
            type Err = PhpDocblockError;

            fn from_str(input: &str) -> Result<Self, Self::Err> {
                Self::new(input)
            }
        }
    };
}

doc_text_newtype!(TagName);
doc_text_newtype!(DocblockTypeString);
doc_text_newtype!(DocblockSummary);
doc_text_newtype!(DocblockBody);

/// Common PHPDoc tag kind metadata.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum DocblockTagKind {
    Param,
    Return,
    Throws,
    Var,
    Property,
    Method,
    Template,
    Deprecated,
    Since,
    See,
    Other,
}

impl DocblockTagKind {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Param => "param",
            Self::Return => "return",
            Self::Throws => "throws",
            Self::Var => "var",
            Self::Property => "property",
            Self::Method => "method",
            Self::Template => "template",
            Self::Deprecated => "deprecated",
            Self::Since => "since",
            Self::See => "see",
            Self::Other => "other",
        }
    }
}

impl fmt::Display for DocblockTagKind {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl FromStr for DocblockTagKind {
    type Err = PhpDocblockError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match normalized_tag(input)?.as_str() {
            "param" => Ok(Self::Param),
            "return" | "returns" => Ok(Self::Return),
            "throws" | "throw" => Ok(Self::Throws),
            "var" => Ok(Self::Var),
            "property" => Ok(Self::Property),
            "method" => Ok(Self::Method),
            "template" => Ok(Self::Template),
            "deprecated" => Ok(Self::Deprecated),
            "since" => Ok(Self::Since),
            "see" => Ok(Self::See),
            _ => Ok(Self::Other),
        }
    }
}

/// Lightweight PHPDoc tag metadata.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DocblockTag {
    name: TagName,
    kind: Option<DocblockTagKind>,
    type_string: Option<DocblockTypeString>,
    description: Option<String>,
}

impl DocblockTag {
    pub fn new(name: TagName) -> Self {
        Self {
            name,
            kind: None,
            type_string: None,
            description: None,
        }
    }

    pub const fn with_kind(mut self, kind: DocblockTagKind) -> Self {
        self.kind = Some(kind);
        self
    }

    pub fn with_type_string(mut self, type_string: DocblockTypeString) -> Self {
        self.type_string = Some(type_string);
        self
    }

    pub fn with_description(mut self, description: &str) -> Self {
        let trimmed = description.trim();
        if !trimmed.is_empty() {
            self.description = Some(trimmed.to_string());
        }
        self
    }

    pub const fn name(&self) -> &TagName {
        &self.name
    }

    pub const fn kind(&self) -> Option<DocblockTagKind> {
        self.kind
    }

    pub const fn type_string(&self) -> Option<&DocblockTypeString> {
        self.type_string.as_ref()
    }

    pub fn description(&self) -> Option<&str> {
        self.description.as_deref()
    }
}

/// PHPDoc block summary/body metadata.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Docblock {
    raw: String,
    summary: String,
    body: String,
}

impl Docblock {
    pub fn new(input: &str) -> Result<Self, PhpDocblockError> {
        let cleaned = clean_docblock_text(input);
        if cleaned.trim().is_empty() {
            return Err(PhpDocblockError::Empty);
        }
        let (summary, body) = split_docblock_summary_body(&cleaned);
        Ok(Self {
            raw: input.to_string(),
            summary,
            body,
        })
    }

    pub fn raw(&self) -> &str {
        &self.raw
    }

    pub fn summary(&self) -> &str {
        &self.summary
    }

    pub fn body(&self) -> &str {
        &self.body
    }
}

/// Error returned when PHPDoc metadata is invalid.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PhpDocblockError {
    Empty,
}

impl fmt::Display for PhpDocblockError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("PHPDoc metadata cannot be empty")
    }
}

impl Error for PhpDocblockError {}

pub fn clean_docblock_text(input: &str) -> String {
    input
        .lines()
        .map(|line| {
            line.trim()
                .trim_start_matches("/**")
                .trim_start_matches("/*")
                .trim_end_matches("*/")
                .trim_start_matches('*')
                .trim()
                .to_string()
        })
        .filter(|line| !line.is_empty())
        .collect::<Vec<_>>()
        .join("\n")
}

pub fn split_docblock_summary_body(input: &str) -> (String, String) {
    let cleaned = clean_docblock_text(input);
    let mut lines = cleaned.lines();
    let summary = lines.next().unwrap_or_default().trim().to_string();
    let body = lines.collect::<Vec<_>>().join("\n").trim().to_string();
    (summary, body)
}

fn normalized_tag(input: &str) -> Result<String, PhpDocblockError> {
    let trimmed = input.trim().trim_start_matches('@');
    if trimmed.is_empty() {
        Err(PhpDocblockError::Empty)
    } else {
        Ok(trimmed.to_ascii_lowercase())
    }
}

#[cfg(test)]
mod tests {
    use super::{
        Docblock, DocblockTag, DocblockTagKind, DocblockTypeString, PhpDocblockError, TagName,
        split_docblock_summary_body,
    };

    #[test]
    fn splits_summary_and_body() -> Result<(), PhpDocblockError> {
        let block = Docblock::new("/**\n * Create a user.\n *\n * Stores metadata only.\n */")?;
        let tag = DocblockTag::new(TagName::new("return")?)
            .with_kind(DocblockTagKind::Return)
            .with_type_string(DocblockTypeString::new("User")?);

        assert_eq!(block.summary(), "Create a user.");
        assert_eq!(block.body(), "Stores metadata only.");
        assert_eq!(tag.kind(), Some(DocblockTagKind::Return));
        assert_eq!(tag.type_string().expect("type").as_str(), "User");
        Ok(())
    }

    #[test]
    fn helper_accepts_plain_text() {
        let (summary, body) = split_docblock_summary_body("Summary.\nBody.");
        assert_eq!(summary, "Summary.");
        assert_eq!(body, "Body.");
    }
}
