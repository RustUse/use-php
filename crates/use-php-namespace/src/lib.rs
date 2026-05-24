#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

use core::{fmt, str::FromStr};
use std::error::Error;

macro_rules! namespace_name_newtype {
    ($name:ident) => {
        #[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
        pub struct $name(String);

        impl $name {
            pub fn new(input: &str) -> Result<Self, PhpNamespaceError> {
                let trimmed = input.trim().trim_matches('\\');
                validate_namespace_path(trimmed)?;
                Ok(Self(trimmed.to_string()))
            }

            pub fn as_str(&self) -> &str {
                &self.0
            }

            pub fn segments(&self) -> Vec<&str> {
                split_segments(self.as_str())
            }
        }

        impl fmt::Display for $name {
            fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                formatter.write_str(self.as_str())
            }
        }

        impl FromStr for $name {
            type Err = PhpNamespaceError;

            fn from_str(input: &str) -> Result<Self, Self::Err> {
                Self::new(input)
            }
        }
    };
}

namespace_name_newtype!(PhpNamespacePath);
namespace_name_newtype!(PhpRelativeName);
namespace_name_newtype!(PhpNamespaceAlias);

impl PhpNamespacePath {
    pub fn global() -> Self {
        Self(String::new())
    }

    pub fn is_global(&self) -> bool {
        self.0.is_empty()
    }

    pub fn join(&self, segment: &str) -> Result<Self, PhpNamespaceError> {
        validate_segment(segment)?;
        if self.is_global() {
            Self::new(segment)
        } else {
            Self::new(&format!("{}\\{segment}", self.as_str()))
        }
    }
}

/// Fully qualified PHP name metadata.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct PhpFullyQualifiedName(String);

impl PhpFullyQualifiedName {
    pub fn new(input: &str) -> Result<Self, PhpNamespaceError> {
        let trimmed = input.trim();
        if !trimmed.starts_with('\\') {
            return Err(PhpNamespaceError::NotFullyQualified);
        }
        let bare = trimmed.trim_start_matches('\\');
        validate_namespace_path(bare)?;
        Ok(Self(bare.to_string()))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }

    pub fn segments(&self) -> Vec<&str> {
        split_segments(self.as_str())
    }
}

impl fmt::Display for PhpFullyQualifiedName {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "\\{}", self.as_str())
    }
}

impl FromStr for PhpFullyQualifiedName {
    type Err = PhpNamespaceError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        Self::new(input)
    }
}

/// PHP namespace import metadata.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PhpUseImport {
    target: PhpFullyQualifiedName,
    alias: Option<PhpNamespaceAlias>,
}

impl PhpUseImport {
    pub const fn new(target: PhpFullyQualifiedName) -> Self {
        Self {
            target,
            alias: None,
        }
    }

    pub fn with_alias(mut self, alias: PhpNamespaceAlias) -> Self {
        self.alias = Some(alias);
        self
    }

    pub const fn target(&self) -> &PhpFullyQualifiedName {
        &self.target
    }

    pub const fn alias(&self) -> Option<&PhpNamespaceAlias> {
        self.alias.as_ref()
    }
}

/// Marker for PHP's global namespace.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct GlobalNamespace;

impl GlobalNamespace {
    pub const fn path(self) -> &'static str {
        ""
    }
}

/// Error returned when PHP namespace metadata is invalid.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PhpNamespaceError {
    Empty,
    EmptySegment,
    InvalidSegment,
    NotFullyQualified,
}

impl fmt::Display for PhpNamespaceError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => formatter.write_str("PHP namespace metadata cannot be empty"),
            Self::EmptySegment => {
                formatter.write_str("PHP namespace cannot contain empty segments")
            },
            Self::InvalidSegment => {
                formatter.write_str("PHP namespace segment has an invalid shape")
            },
            Self::NotFullyQualified => formatter.write_str("PHP name is not fully qualified"),
        }
    }
}

impl Error for PhpNamespaceError {}

fn split_segments(input: &str) -> Vec<&str> {
    input
        .split('\\')
        .filter(|segment| !segment.is_empty())
        .collect()
}

fn validate_namespace_path(input: &str) -> Result<(), PhpNamespaceError> {
    if input.is_empty() {
        return Ok(());
    }
    for segment in input.split('\\') {
        if segment.is_empty() {
            return Err(PhpNamespaceError::EmptySegment);
        }
        validate_segment(segment)?;
    }
    Ok(())
}

fn validate_segment(input: &str) -> Result<(), PhpNamespaceError> {
    let trimmed = input.trim();
    if trimmed.is_empty() {
        return Err(PhpNamespaceError::Empty);
    }
    let mut characters = trimmed.chars();
    let Some(first) = characters.next() else {
        return Err(PhpNamespaceError::Empty);
    };
    if (first == '_' || first.is_ascii_alphabetic())
        && characters.all(|character| character == '_' || character.is_ascii_alphanumeric())
    {
        Ok(())
    } else {
        Err(PhpNamespaceError::InvalidSegment)
    }
}

#[cfg(test)]
mod tests {
    use super::{
        GlobalNamespace, PhpFullyQualifiedName, PhpNamespaceAlias, PhpNamespaceError,
        PhpNamespacePath, PhpUseImport,
    };

    #[test]
    fn validates_namespace_paths() -> Result<(), PhpNamespaceError> {
        let namespace = PhpNamespacePath::new("App\\Http")?.join("Controller")?;
        let name = PhpFullyQualifiedName::new("\\App\\Http\\Controller\\HomeController")?;
        let import = PhpUseImport::new(name).with_alias(PhpNamespaceAlias::new("HomeController")?);

        assert_eq!(namespace.segments(), vec!["App", "Http", "Controller"]);
        assert_eq!(import.alias().expect("alias").as_str(), "HomeController");
        assert_eq!(GlobalNamespace.path(), "");
        Ok(())
    }

    #[test]
    fn supports_global_namespace() {
        assert!(PhpNamespacePath::global().is_global());
    }
}
