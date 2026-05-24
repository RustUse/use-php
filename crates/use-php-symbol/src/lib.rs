#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

use core::{fmt, str::FromStr};
use std::error::Error;

/// PHP symbol kind metadata.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum SymbolKind {
    Class,
    Interface,
    Trait,
    Enum,
    Function,
    Constant,
    Method,
    Property,
    Parameter,
}

impl SymbolKind {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Class => "class",
            Self::Interface => "interface",
            Self::Trait => "trait",
            Self::Enum => "enum",
            Self::Function => "function",
            Self::Constant => "constant",
            Self::Method => "method",
            Self::Property => "property",
            Self::Parameter => "parameter",
        }
    }
}

impl fmt::Display for SymbolKind {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl FromStr for SymbolKind {
    type Err = PhpSymbolError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match normalized_label(input)?.as_str() {
            "class" => Ok(Self::Class),
            "interface" => Ok(Self::Interface),
            "trait" => Ok(Self::Trait),
            "enum" => Ok(Self::Enum),
            "function" => Ok(Self::Function),
            "constant" | "const" => Ok(Self::Constant),
            "method" => Ok(Self::Method),
            "property" => Ok(Self::Property),
            "parameter" | "param" => Ok(Self::Parameter),
            _ => Err(PhpSymbolError::UnknownLabel),
        }
    }
}

/// PHP class-like symbol kind metadata.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum PhpClassLikeKind {
    Class,
    Interface,
    Trait,
    Enum,
}

impl PhpClassLikeKind {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Class => "class",
            Self::Interface => "interface",
            Self::Trait => "trait",
            Self::Enum => "enum",
        }
    }
}

/// PHP member kind metadata.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum PhpMemberKind {
    Method,
    Property,
    Constant,
    Case,
}

impl PhpMemberKind {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Method => "method",
            Self::Property => "property",
            Self::Constant => "constant",
            Self::Case => "case",
        }
    }
}

/// Lightly validated PHP symbol name metadata.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct SymbolName(String);

impl SymbolName {
    pub fn new(input: &str) -> Result<Self, PhpSymbolError> {
        let trimmed = input.trim();
        if trimmed.is_empty() {
            return Err(PhpSymbolError::Empty);
        }
        if !is_valid_php_symbol_name(trimmed) {
            return Err(PhpSymbolError::InvalidName);
        }
        Ok(Self(trimmed.to_string()))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }

    pub fn bare_name(&self) -> &str {
        self.0.strip_prefix('$').unwrap_or(self.as_str())
    }
}

impl fmt::Display for SymbolName {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl FromStr for SymbolName {
    type Err = PhpSymbolError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        Self::new(input)
    }
}

/// PHP symbol metadata.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PhpSymbol {
    kind: SymbolKind,
    name: SymbolName,
}

impl PhpSymbol {
    pub const fn new(kind: SymbolKind, name: SymbolName) -> Self {
        Self { kind, name }
    }

    pub const fn kind(&self) -> SymbolKind {
        self.kind
    }

    pub const fn name(&self) -> &SymbolName {
        &self.name
    }
}

/// Error returned when PHP symbol metadata is invalid.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PhpSymbolError {
    Empty,
    InvalidName,
    UnknownLabel,
}

impl fmt::Display for PhpSymbolError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => formatter.write_str("PHP symbol name cannot be empty"),
            Self::InvalidName => formatter.write_str("PHP symbol name has an invalid shape"),
            Self::UnknownLabel => formatter.write_str("unknown PHP symbol metadata label"),
        }
    }
}

impl Error for PhpSymbolError {}

pub fn is_valid_php_symbol_name(input: &str) -> bool {
    let trimmed = input.trim();
    let bare = trimmed.strip_prefix('$').unwrap_or(trimmed);
    let mut characters = bare.chars();
    let Some(first) = characters.next() else {
        return false;
    };
    (first == '_' || first.is_ascii_alphabetic())
        && characters.all(|character| character == '_' || character.is_ascii_alphanumeric())
}

fn normalized_label(input: &str) -> Result<String, PhpSymbolError> {
    let trimmed = input.trim();
    if trimmed.is_empty() {
        Err(PhpSymbolError::Empty)
    } else {
        Ok(trimmed.to_ascii_lowercase().replace(['-', '_', ' '], ""))
    }
}

#[cfg(test)]
mod tests {
    use super::{
        PhpClassLikeKind, PhpSymbol, PhpSymbolError, SymbolKind, SymbolName,
        is_valid_php_symbol_name,
    };

    #[test]
    fn validates_symbol_names() -> Result<(), PhpSymbolError> {
        let name = SymbolName::new(" $value ")?;
        let symbol = PhpSymbol::new(SymbolKind::Parameter, name);

        assert_eq!(symbol.name().as_str(), "$value");
        assert_eq!(symbol.name().bare_name(), "value");
        assert!(is_valid_php_symbol_name("ExampleController"));
        assert!(!is_valid_php_symbol_name("123bad"));
        Ok(())
    }

    #[test]
    fn exposes_class_like_labels() {
        assert_eq!(PhpClassLikeKind::Interface.as_str(), "interface");
        assert_eq!(SymbolKind::Method.to_string(), "method");
    }
}
