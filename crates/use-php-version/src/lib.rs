#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

use core::{fmt, str::FromStr};
use std::error::Error;

/// PHP major version component.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct PhpMajorVersion(u16);

impl PhpMajorVersion {
    pub const fn new(value: u16) -> Result<Self, PhpVersionParseError> {
        if value == 0 {
            Err(PhpVersionParseError::InvalidVersion)
        } else {
            Ok(Self(value))
        }
    }

    pub const fn get(self) -> u16 {
        self.0
    }
}

/// PHP minor version component.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct PhpMinorVersion(u16);

impl PhpMinorVersion {
    pub const fn new(value: u16) -> Self {
        Self(value)
    }

    pub const fn get(self) -> u16 {
        self.0
    }
}

/// PHP patch version component.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct PhpPatchVersion(u16);

impl PhpPatchVersion {
    pub const fn new(value: u16) -> Self {
        Self(value)
    }

    pub const fn get(self) -> u16 {
        self.0
    }
}

/// Lightweight PHP version metadata.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct PhpVersion {
    major: PhpMajorVersion,
    minor: Option<PhpMinorVersion>,
    patch: Option<PhpPatchVersion>,
    suffix: Option<String>,
}

impl PhpVersion {
    pub fn new(
        major: u16,
        minor: Option<u16>,
        patch: Option<u16>,
    ) -> Result<Self, PhpVersionParseError> {
        if minor.is_none() && patch.is_some() {
            return Err(PhpVersionParseError::InvalidVersion);
        }

        Ok(Self {
            major: PhpMajorVersion::new(major)?,
            minor: minor.map(PhpMinorVersion::new),
            patch: patch.map(PhpPatchVersion::new),
            suffix: None,
        })
    }

    pub const fn major(&self) -> u16 {
        self.major.get()
    }

    pub const fn minor(&self) -> Option<u16> {
        match self.minor {
            Some(value) => Some(value.get()),
            None => None,
        }
    }

    pub const fn patch(&self) -> Option<u16> {
        match self.patch {
            Some(value) => Some(value.get()),
            None => None,
        }
    }

    pub fn suffix(&self) -> Option<&str> {
        self.suffix.as_deref()
    }

    pub const fn is_php8_or_newer(&self) -> bool {
        self.major() >= 8
    }
}

impl fmt::Display for PhpVersion {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}", self.major())?;
        if let Some(minor) = self.minor() {
            write!(formatter, ".{minor}")?;
        }
        if let Some(patch) = self.patch() {
            write!(formatter, ".{patch}")?;
        }
        if let Some(suffix) = self.suffix() {
            formatter.write_str(suffix)?;
        }
        Ok(())
    }
}

impl FromStr for PhpVersion {
    type Err = PhpVersionParseError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        parse_php_version(input)
    }
}

/// PHP minor release branch metadata such as `8.3`.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct PhpVersionBranch {
    major: PhpMajorVersion,
    minor: PhpMinorVersion,
}

impl PhpVersionBranch {
    pub fn new(major: u16, minor: u16) -> Result<Self, PhpVersionParseError> {
        Ok(Self {
            major: PhpMajorVersion::new(major)?,
            minor: PhpMinorVersion::new(minor),
        })
    }

    pub fn from_version(version: &PhpVersion) -> Result<Self, PhpVersionParseError> {
        let Some(minor) = version.minor() else {
            return Err(PhpVersionParseError::InvalidVersion);
        };
        Self::new(version.major(), minor)
    }

    pub const fn major(self) -> u16 {
        self.major.get()
    }

    pub const fn minor(self) -> u16 {
        self.minor.get()
    }
}

impl fmt::Display for PhpVersionBranch {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}.{}", self.major(), self.minor())
    }
}

/// Static support phase labels for PHP version metadata.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum PhpSupportPhase {
    Active,
    Security,
    EndOfLife,
    Unknown,
}

impl PhpSupportPhase {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Active => "active",
            Self::Security => "security",
            Self::EndOfLife => "end-of-life",
            Self::Unknown => "unknown",
        }
    }
}

impl fmt::Display for PhpSupportPhase {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl FromStr for PhpSupportPhase {
    type Err = PhpVersionParseError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match normalized_label(input)?.as_str() {
            "active" => Ok(Self::Active),
            "security" | "securityonly" => Ok(Self::Security),
            "endoflife" | "eol" => Ok(Self::EndOfLife),
            "unknown" => Ok(Self::Unknown),
            _ => Err(PhpVersionParseError::UnknownLabel),
        }
    }
}

/// Error returned when PHP version metadata cannot be parsed.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PhpVersionParseError {
    Empty,
    InvalidNumber,
    InvalidVersion,
    TooManyComponents,
    UnknownLabel,
}

impl fmt::Display for PhpVersionParseError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => formatter.write_str("PHP version metadata cannot be empty"),
            Self::InvalidNumber => formatter.write_str("PHP version contains an invalid number"),
            Self::InvalidVersion => formatter.write_str("PHP version has an invalid shape"),
            Self::TooManyComponents => formatter.write_str("PHP version has too many components"),
            Self::UnknownLabel => formatter.write_str("unknown PHP version metadata label"),
        }
    }
}

impl Error for PhpVersionParseError {}

pub fn parse_php_version(input: &str) -> Result<PhpVersion, PhpVersionParseError> {
    let trimmed = input.trim();
    if trimmed.is_empty() {
        return Err(PhpVersionParseError::Empty);
    }

    let suffix_start = trimmed.char_indices().find_map(|(index, character)| {
        (!character.is_ascii_digit() && character != '.').then_some(index)
    });
    let (core, suffix) = match suffix_start {
        Some(index) => (&trimmed[..index], Some(trimmed[index..].to_string())),
        None => (trimmed, None),
    };
    if core.is_empty() || core.ends_with('.') {
        return Err(PhpVersionParseError::InvalidVersion);
    }

    let parts = core.split('.').collect::<Vec<_>>();
    if parts.len() > 3 {
        return Err(PhpVersionParseError::TooManyComponents);
    }
    if parts.iter().any(|part| part.is_empty()) {
        return Err(PhpVersionParseError::InvalidVersion);
    }

    let major = parts[0]
        .parse::<u16>()
        .map_err(|_| PhpVersionParseError::InvalidNumber)?;
    let minor = parts
        .get(1)
        .map(|part| {
            part.parse::<u16>()
                .map_err(|_| PhpVersionParseError::InvalidNumber)
        })
        .transpose()?;
    let patch = parts
        .get(2)
        .map(|part| {
            part.parse::<u16>()
                .map_err(|_| PhpVersionParseError::InvalidNumber)
        })
        .transpose()?;

    let mut version = PhpVersion::new(major, minor, patch)?;
    version.suffix = suffix;
    Ok(version)
}

fn normalized_label(input: &str) -> Result<String, PhpVersionParseError> {
    let trimmed = input.trim();
    if trimmed.is_empty() {
        Err(PhpVersionParseError::Empty)
    } else {
        Ok(trimmed.to_ascii_lowercase().replace(['-', '_', ' '], ""))
    }
}

#[cfg(test)]
mod tests {
    use super::{PhpSupportPhase, PhpVersion, PhpVersionBranch, PhpVersionParseError};

    #[test]
    fn parses_version_and_branch() -> Result<(), PhpVersionParseError> {
        let version: PhpVersion = "8.3.2RC1".parse()?;
        let branch = PhpVersionBranch::from_version(&version)?;

        assert_eq!(version.major(), 8);
        assert_eq!(version.minor(), Some(3));
        assert_eq!(version.patch(), Some(2));
        assert_eq!(version.suffix(), Some("RC1"));
        assert_eq!(branch.to_string(), "8.3");
        assert!(version.is_php8_or_newer());
        Ok(())
    }

    #[test]
    fn parses_support_phase_labels() -> Result<(), PhpVersionParseError> {
        assert_eq!(
            "security-only".parse::<PhpSupportPhase>()?,
            PhpSupportPhase::Security
        );
        assert_eq!(PhpSupportPhase::EndOfLife.to_string(), "end-of-life");
        Ok(())
    }
}
