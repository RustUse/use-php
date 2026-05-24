#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

use core::{fmt, str::FromStr};
use std::error::Error;

macro_rules! laravel_text_newtype {
    ($name:ident) => {
        #[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
        pub struct $name(String);

        impl $name {
            pub fn new(input: &str) -> Result<Self, LaravelError> {
                let trimmed = input.trim();
                if trimmed.is_empty() {
                    Err(LaravelError::Empty)
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
            type Err = LaravelError;

            fn from_str(input: &str) -> Result<Self, Self::Err> {
                Self::new(input)
            }
        }
    };
}

laravel_text_newtype!(LaravelRouteName);
laravel_text_newtype!(LaravelMiddlewareName);
laravel_text_newtype!(ArtisanCommandName);
laravel_text_newtype!(LaravelMigrationName);
laravel_text_newtype!(ServiceProviderName);
laravel_text_newtype!(LaravelConfigKey);

/// Laravel metadata kind.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum LaravelMetadataKind {
    Route,
    Middleware,
    ArtisanCommand,
    Migration,
    ServiceProvider,
    ConfigKey,
}

impl LaravelMetadataKind {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Route => "route",
            Self::Middleware => "middleware",
            Self::ArtisanCommand => "artisan-command",
            Self::Migration => "migration",
            Self::ServiceProvider => "service-provider",
            Self::ConfigKey => "config-key",
        }
    }
}

/// Laravel metadata reference.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LaravelMetadata {
    name: String,
    kind: LaravelMetadataKind,
}

impl LaravelMetadata {
    pub fn new(name: &str, kind: LaravelMetadataKind) -> Result<Self, LaravelError> {
        let trimmed = name.trim();
        if trimmed.is_empty() {
            Err(LaravelError::Empty)
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

    pub const fn kind(&self) -> LaravelMetadataKind {
        self.kind
    }
}

/// Error returned when Laravel metadata is invalid.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum LaravelError {
    Empty,
}

impl fmt::Display for LaravelError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("Laravel metadata cannot be empty")
    }
}

impl Error for LaravelError {}

#[cfg(test)]
mod tests {
    use super::{
        ArtisanCommandName, LaravelConfigKey, LaravelError, LaravelMetadata, LaravelMetadataKind,
        LaravelRouteName,
    };

    #[test]
    fn builds_laravel_metadata() -> Result<(), LaravelError> {
        let route = LaravelRouteName::new("books.index")?;
        let command = ArtisanCommandName::new("books:sync")?;
        let config = LaravelConfigKey::new("cache.default")?;
        let metadata = LaravelMetadata::new(route.as_str(), LaravelMetadataKind::Route)?;

        assert_eq!(route.as_str(), "books.index");
        assert_eq!(command.as_str(), "books:sync");
        assert_eq!(config.as_str(), "cache.default");
        assert_eq!(metadata.kind().as_str(), "route");
        Ok(())
    }
}
