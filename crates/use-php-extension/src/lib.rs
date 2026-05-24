#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

use core::{fmt, str::FromStr};
use std::error::Error;

macro_rules! extension_text_newtype {
    ($name:ident) => {
        #[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
        pub struct $name(String);

        impl $name {
            pub fn new(input: &str) -> Result<Self, PhpExtensionError> {
                let trimmed = input.trim();
                if trimmed.is_empty() {
                    Err(PhpExtensionError::Empty)
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
            type Err = PhpExtensionError;

            fn from_str(input: &str) -> Result<Self, Self::Err> {
                Self::new(input)
            }
        }
    };
}

extension_text_newtype!(PhpExtensionName);
extension_text_newtype!(PhpVersionConstraint);

/// PHP extension kind metadata.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum PhpExtensionKind {
    Core,
    Bundled,
    Pecl,
    Zend,
    Userland,
    Unknown,
}

impl PhpExtensionKind {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Core => "core",
            Self::Bundled => "bundled",
            Self::Pecl => "pecl",
            Self::Zend => "zend",
            Self::Userland => "userland",
            Self::Unknown => "unknown",
        }
    }
}

/// PHP extension requirement kind metadata.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum PhpExtensionRequirementKind {
    Required,
    Optional,
    Conflict,
}

impl PhpExtensionRequirementKind {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Required => "required",
            Self::Optional => "optional",
            Self::Conflict => "conflict",
        }
    }
}

/// PHP extension requirement metadata.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PhpExtensionRequirement {
    name: PhpExtensionName,
    requirement_kind: PhpExtensionRequirementKind,
    extension_kind: PhpExtensionKind,
    version: Option<PhpVersionConstraint>,
}

impl PhpExtensionRequirement {
    pub const fn required(name: PhpExtensionName) -> Self {
        Self {
            name,
            requirement_kind: PhpExtensionRequirementKind::Required,
            extension_kind: PhpExtensionKind::Unknown,
            version: None,
        }
    }

    pub const fn optional(name: PhpExtensionName) -> Self {
        Self {
            name,
            requirement_kind: PhpExtensionRequirementKind::Optional,
            extension_kind: PhpExtensionKind::Unknown,
            version: None,
        }
    }

    pub const fn with_kind(mut self, extension_kind: PhpExtensionKind) -> Self {
        self.extension_kind = extension_kind;
        self
    }

    pub fn with_version(mut self, version: PhpVersionConstraint) -> Self {
        self.version = Some(version);
        self
    }

    pub const fn name(&self) -> &PhpExtensionName {
        &self.name
    }

    pub const fn requirement_kind(&self) -> PhpExtensionRequirementKind {
        self.requirement_kind
    }

    pub const fn extension_kind(&self) -> PhpExtensionKind {
        self.extension_kind
    }

    pub const fn version(&self) -> Option<&PhpVersionConstraint> {
        self.version.as_ref()
    }

    pub const fn is_required(&self) -> bool {
        matches!(self.requirement_kind, PhpExtensionRequirementKind::Required)
    }
}

/// Error returned when PHP extension metadata is invalid.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PhpExtensionError {
    Empty,
}

impl fmt::Display for PhpExtensionError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("PHP extension metadata cannot be empty")
    }
}

impl Error for PhpExtensionError {}

#[cfg(test)]
mod tests {
    use super::{
        PhpExtensionError, PhpExtensionKind, PhpExtensionName, PhpExtensionRequirement,
        PhpVersionConstraint,
    };

    #[test]
    fn builds_extension_requirement() -> Result<(), PhpExtensionError> {
        let requirement = PhpExtensionRequirement::required(PhpExtensionName::new("mbstring")?)
            .with_kind(PhpExtensionKind::Bundled)
            .with_version(PhpVersionConstraint::new("*")?);

        assert_eq!(requirement.name().as_str(), "mbstring");
        assert!(requirement.is_required());
        assert_eq!(requirement.extension_kind(), PhpExtensionKind::Bundled);
        Ok(())
    }
}
