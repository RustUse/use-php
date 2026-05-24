#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

use core::{fmt, str::FromStr};
use std::error::Error;

macro_rules! packagist_text_newtype {
    ($name:ident) => {
        #[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
        pub struct $name(String);

        impl $name {
            pub fn new(input: &str) -> Result<Self, PackagistError> {
                let trimmed = input.trim();
                if trimmed.is_empty() {
                    Err(PackagistError::Empty)
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
    };
}

packagist_text_newtype!(PackagistVendorName);
packagist_text_newtype!(PackagistPackageShortName);
packagist_text_newtype!(PackagistMetadataLabel);

/// Packagist package name metadata in `vendor/package` form.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct PackagistPackageName {
    vendor: String,
    package: String,
}

impl PackagistPackageName {
    pub fn new(input: &str) -> Result<Self, PackagistError> {
        let trimmed = input.trim();
        let Some((vendor, package)) = trimmed.split_once('/') else {
            return Err(PackagistError::InvalidPackageName);
        };
        if vendor.is_empty() || package.is_empty() || package.contains('/') {
            return Err(PackagistError::InvalidPackageName);
        }
        Ok(Self {
            vendor: vendor.to_string(),
            package: package.to_string(),
        })
    }

    pub fn vendor(&self) -> &str {
        &self.vendor
    }

    pub fn package(&self) -> &str {
        &self.package
    }
}

impl fmt::Display for PackagistPackageName {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}/{}", self.vendor, self.package)
    }
}

impl FromStr for PackagistPackageName {
    type Err = PackagistError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        Self::new(input)
    }
}

/// Packagist package type metadata.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum PackagistPackageType {
    Library,
    Project,
    Metapackage,
    ComposerPlugin,
    SymfonyBundle,
    Other,
}

impl PackagistPackageType {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Library => "library",
            Self::Project => "project",
            Self::Metapackage => "metapackage",
            Self::ComposerPlugin => "composer-plugin",
            Self::SymfonyBundle => "symfony-bundle",
            Self::Other => "other",
        }
    }
}

/// Packagist stability metadata.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum PackagistStability {
    Dev,
    Alpha,
    Beta,
    Rc,
    Stable,
}

impl PackagistStability {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Dev => "dev",
            Self::Alpha => "alpha",
            Self::Beta => "beta",
            Self::Rc => "RC",
            Self::Stable => "stable",
        }
    }
}

/// Packagist download count metadata.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct PackagistDownloadCount(u64);

impl PackagistDownloadCount {
    pub const fn new(value: u64) -> Self {
        Self(value)
    }

    pub const fn get(self) -> u64 {
        self.0
    }
}

/// Error returned when Packagist metadata is invalid.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PackagistError {
    Empty,
    InvalidPackageName,
}

impl fmt::Display for PackagistError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => formatter.write_str("Packagist metadata cannot be empty"),
            Self::InvalidPackageName => {
                formatter.write_str("Packagist package name must look like vendor/package")
            },
        }
    }
}

impl Error for PackagistError {}

#[cfg(test)]
mod tests {
    use super::{
        PackagistDownloadCount, PackagistError, PackagistPackageName, PackagistPackageType,
        PackagistStability,
    };

    #[test]
    fn validates_package_name_and_labels() -> Result<(), PackagistError> {
        let package = PackagistPackageName::new("symfony/console")?;
        let downloads = PackagistDownloadCount::new(42);

        assert_eq!(package.vendor(), "symfony");
        assert_eq!(downloads.get(), 42);
        assert_eq!(PackagistPackageType::Library.as_str(), "library");
        assert_eq!(PackagistStability::Stable.as_str(), "stable");
        Ok(())
    }
}
