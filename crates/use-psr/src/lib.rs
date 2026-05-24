#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

use core::fmt;
use std::error::Error;

/// PHP-FIG PSR number metadata.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct PsrNumber(u16);

impl PsrNumber {
    pub const fn new(value: u16) -> Result<Self, PsrError> {
        if value == 0 {
            Err(PsrError::InvalidNumber)
        } else {
            Ok(Self(value))
        }
    }

    pub const fn get(self) -> u16 {
        self.0
    }
}

impl fmt::Display for PsrNumber {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "PSR-{}", self.0)
    }
}

/// PSR title metadata.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct PsrTitle(String);

impl PsrTitle {
    pub fn new(input: &str) -> Result<Self, PsrError> {
        let trimmed = input.trim();
        if trimmed.is_empty() {
            Err(PsrError::Empty)
        } else {
            Ok(Self(trimmed.to_string()))
        }
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// PSR status metadata.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum PsrStatus {
    Draft,
    Accepted,
    Deprecated,
    Abandoned,
    Unknown,
}

impl PsrStatus {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Draft => "draft",
            Self::Accepted => "accepted",
            Self::Deprecated => "deprecated",
            Self::Abandoned => "abandoned",
            Self::Unknown => "unknown",
        }
    }
}

/// Broad PSR category metadata.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum PsrCategory {
    Autoloading,
    CodingStyle,
    Http,
    Logging,
    Cache,
    Container,
    Events,
    Security,
    Other,
}

impl PsrCategory {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Autoloading => "autoloading",
            Self::CodingStyle => "coding-style",
            Self::Http => "http",
            Self::Logging => "logging",
            Self::Cache => "cache",
            Self::Container => "container",
            Self::Events => "events",
            Self::Security => "security",
            Self::Other => "other",
        }
    }
}

/// PHP-FIG PSR metadata record.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PsrMetadata {
    number: PsrNumber,
    title: PsrTitle,
    status: PsrStatus,
    category: PsrCategory,
}

impl PsrMetadata {
    pub const fn new(
        number: PsrNumber,
        title: PsrTitle,
        status: PsrStatus,
        category: PsrCategory,
    ) -> Self {
        Self {
            number,
            title,
            status,
            category,
        }
    }

    pub const fn number(&self) -> PsrNumber {
        self.number
    }

    pub const fn title(&self) -> &PsrTitle {
        &self.title
    }

    pub const fn status(&self) -> PsrStatus {
        self.status
    }

    pub const fn category(&self) -> PsrCategory {
        self.category
    }

    pub fn identifier(&self) -> String {
        self.number.to_string()
    }
}

/// Error returned when PSR metadata is invalid.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PsrError {
    Empty,
    InvalidNumber,
}

impl fmt::Display for PsrError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => formatter.write_str("PSR metadata cannot be empty"),
            Self::InvalidNumber => formatter.write_str("PSR number must be non-zero"),
        }
    }
}

impl Error for PsrError {}

#[cfg(test)]
mod tests {
    use super::{PsrCategory, PsrError, PsrMetadata, PsrNumber, PsrStatus, PsrTitle};

    #[test]
    fn builds_psr_metadata() -> Result<(), PsrError> {
        let metadata = PsrMetadata::new(
            PsrNumber::new(4)?,
            PsrTitle::new("Autoloading Standard")?,
            PsrStatus::Accepted,
            PsrCategory::Autoloading,
        );

        assert_eq!(metadata.identifier(), "PSR-4");
        assert_eq!(metadata.category().as_str(), "autoloading");
        Ok(())
    }
}
