#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

use core::{fmt, str::FromStr};
use std::error::Error;

macro_rules! drupal_text_newtype {
    ($name:ident) => {
        #[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
        pub struct $name(String);

        impl $name {
            pub fn new(input: &str) -> Result<Self, DrupalError> {
                let trimmed = input.trim();
                if trimmed.is_empty() {
                    Err(DrupalError::Empty)
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
            type Err = DrupalError;

            fn from_str(input: &str) -> Result<Self, Self::Err> {
                Self::new(input)
            }
        }
    };
}

drupal_text_newtype!(DrupalModuleName);
drupal_text_newtype!(DrupalThemeName);
drupal_text_newtype!(DrupalRouteName);
drupal_text_newtype!(DrupalEntityTypeId);
drupal_text_newtype!(DrupalConfigObjectName);
drupal_text_newtype!(DrupalPermission);

/// Drupal extension kind metadata.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum DrupalExtensionKind {
    Module,
    Theme,
    Profile,
}

impl DrupalExtensionKind {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Module => "module",
            Self::Theme => "theme",
            Self::Profile => "profile",
        }
    }
}

/// Drupal metadata reference.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DrupalMetadataReference {
    name: String,
    kind: DrupalExtensionKind,
}

impl DrupalMetadataReference {
    pub fn new(name: &str, kind: DrupalExtensionKind) -> Result<Self, DrupalError> {
        let trimmed = name.trim();
        if trimmed.is_empty() {
            Err(DrupalError::Empty)
        } else {
            Ok(Self {
                name: trimmed.to_string(),
                kind,
            })
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub const fn kind(&self) -> DrupalExtensionKind {
        self.kind
    }
}

/// Error returned when Drupal metadata is invalid.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum DrupalError {
    Empty,
}

impl fmt::Display for DrupalError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("Drupal metadata cannot be empty")
    }
}

impl Error for DrupalError {}

#[cfg(test)]
mod tests {
    use super::{
        DrupalError, DrupalExtensionKind, DrupalMetadataReference, DrupalModuleName,
        DrupalPermission, DrupalRouteName,
    };

    #[test]
    fn builds_drupal_metadata() -> Result<(), DrupalError> {
        let module = DrupalModuleName::new("book_tools")?;
        let route = DrupalRouteName::new("book_tools.index")?;
        let permission = DrupalPermission::new("administer book tools")?;
        let reference = DrupalMetadataReference::new("book_tools", DrupalExtensionKind::Module)?;

        assert_eq!(module.as_str(), "book_tools");
        assert_eq!(route.as_str(), "book_tools.index");
        assert_eq!(permission.as_str(), "administer book tools");
        assert_eq!(reference.kind().as_str(), "module");
        Ok(())
    }
}
