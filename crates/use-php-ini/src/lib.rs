#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

use core::{fmt, str::FromStr};
use std::error::Error;

macro_rules! ini_text_newtype {
    ($name:ident) => {
        #[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
        pub struct $name(String);

        impl $name {
            pub fn new(input: &str) -> Result<Self, PhpIniError> {
                let trimmed = input.trim();
                if trimmed.is_empty() {
                    Err(PhpIniError::Empty)
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
            type Err = PhpIniError;

            fn from_str(input: &str) -> Result<Self, Self::Err> {
                Self::new(input)
            }
        }
    };
}

ini_text_newtype!(PhpIniSectionName);
ini_text_newtype!(PhpIniDirectiveName);

/// PHP INI environment label metadata.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum PhpIniEnvironment {
    Production,
    Development,
    Testing,
    Cli,
    Unknown,
}

impl PhpIniEnvironment {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Production => "production",
            Self::Development => "development",
            Self::Testing => "testing",
            Self::Cli => "cli",
            Self::Unknown => "unknown",
        }
    }
}

impl fmt::Display for PhpIniEnvironment {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

/// Scalar PHP INI value metadata.
#[derive(Clone, Debug, PartialEq)]
pub enum PhpIniValue {
    String(String),
    Integer(i64),
    Float(f64),
    Boolean(bool),
    Null,
}

impl fmt::Display for PhpIniValue {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::String(value) => formatter.write_str(value),
            Self::Integer(value) => write!(formatter, "{value}"),
            Self::Float(value) => write!(formatter, "{value}"),
            Self::Boolean(value) => formatter.write_str(if *value { "true" } else { "false" }),
            Self::Null => formatter.write_str("null"),
        }
    }
}

impl FromStr for PhpIniValue {
    type Err = PhpIniError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let trimmed = input.trim().trim_matches('"');
        if trimmed.is_empty() {
            return Ok(Self::String(String::new()));
        }
        match trimmed.to_ascii_lowercase().as_str() {
            "true" | "on" | "yes" => Ok(Self::Boolean(true)),
            "false" | "off" | "no" => Ok(Self::Boolean(false)),
            "null" | "none" => Ok(Self::Null),
            _ => trimmed
                .parse::<i64>()
                .map(Self::Integer)
                .or_else(|_| trimmed.parse::<f64>().map(Self::Float))
                .or_else(|_| Ok(Self::String(trimmed.to_string()))),
        }
    }
}

/// PHP INI directive metadata.
#[derive(Clone, Debug, PartialEq)]
pub struct PhpIniDirective {
    section: Option<PhpIniSectionName>,
    name: PhpIniDirectiveName,
    value: PhpIniValue,
}

impl PhpIniDirective {
    pub const fn new(name: PhpIniDirectiveName, value: PhpIniValue) -> Self {
        Self {
            section: None,
            name,
            value,
        }
    }

    pub fn with_section(mut self, section: PhpIniSectionName) -> Self {
        self.section = Some(section);
        self
    }

    pub const fn section(&self) -> Option<&PhpIniSectionName> {
        self.section.as_ref()
    }

    pub const fn name(&self) -> &PhpIniDirectiveName {
        &self.name
    }

    pub const fn value(&self) -> &PhpIniValue {
        &self.value
    }
}

impl fmt::Display for PhpIniDirective {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{} = {}", self.name, self.value)
    }
}

/// Error returned when PHP INI metadata is invalid.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PhpIniError {
    Empty,
    MissingEquals,
}

impl fmt::Display for PhpIniError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => formatter.write_str("PHP INI metadata cannot be empty"),
            Self::MissingEquals => formatter.write_str("PHP INI directive line must contain '='"),
        }
    }
}

impl Error for PhpIniError {}

pub fn parse_ini_directive_line(input: &str) -> Result<PhpIniDirective, PhpIniError> {
    let trimmed = input.trim();
    if trimmed.is_empty() {
        return Err(PhpIniError::Empty);
    }
    let Some((name, value)) = trimmed.split_once('=') else {
        return Err(PhpIniError::MissingEquals);
    };
    Ok(PhpIniDirective::new(
        PhpIniDirectiveName::new(name)?,
        value.parse()?,
    ))
}

#[cfg(test)]
mod tests {
    use super::{
        PhpIniDirective, PhpIniDirectiveName, PhpIniEnvironment, PhpIniError, PhpIniValue,
        parse_ini_directive_line,
    };

    #[test]
    fn formats_and_parses_directives() -> Result<(), PhpIniError> {
        let directive = PhpIniDirective::new(
            PhpIniDirectiveName::new("memory_limit")?,
            PhpIniValue::String("128M".to_string()),
        );
        let parsed = parse_ini_directive_line("display_errors = On")?;

        assert_eq!(directive.to_string(), "memory_limit = 128M");
        assert_eq!(parsed.value(), &PhpIniValue::Boolean(true));
        assert_eq!(PhpIniEnvironment::Development.to_string(), "development");
        Ok(())
    }
}
