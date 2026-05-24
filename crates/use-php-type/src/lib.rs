#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

use core::{fmt, str::FromStr};
use std::error::Error;

/// PHP scalar type metadata.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum PhpScalarType {
    Bool,
    Int,
    Float,
    String,
}

impl PhpScalarType {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Bool => "bool",
            Self::Int => "int",
            Self::Float => "float",
            Self::String => "string",
        }
    }
}

impl fmt::Display for PhpScalarType {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl FromStr for PhpScalarType {
    type Err = PhpTypeError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match normalized_label(input)?.as_str() {
            "bool" | "boolean" => Ok(Self::Bool),
            "int" | "integer" => Ok(Self::Int),
            "float" | "double" => Ok(Self::Float),
            "string" => Ok(Self::String),
            _ => Err(PhpTypeError::UnknownLabel),
        }
    }
}

/// Broad PHP type kind metadata.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum PhpTypeKind {
    Scalar,
    Nullable,
    Union,
    Intersection,
    Mixed,
    Never,
    Void,
    Callable,
    Iterable,
    Object,
    ClassLike,
}

impl PhpTypeKind {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Scalar => "scalar",
            Self::Nullable => "nullable",
            Self::Union => "union",
            Self::Intersection => "intersection",
            Self::Mixed => "mixed",
            Self::Never => "never",
            Self::Void => "void",
            Self::Callable => "callable",
            Self::Iterable => "iterable",
            Self::Object => "object",
            Self::ClassLike => "class-like",
        }
    }
}

impl fmt::Display for PhpTypeKind {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

/// Lightly validated PHP type name metadata.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct PhpTypeName(String);

impl PhpTypeName {
    pub fn new(input: &str) -> Result<Self, PhpTypeError> {
        let trimmed = input.trim().trim_start_matches('\\');
        validate_type_name(trimmed)?;
        Ok(Self(trimmed.to_string()))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }

    pub fn segments(&self) -> Vec<&str> {
        self.0.split('\\').collect()
    }
}

impl fmt::Display for PhpTypeName {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl FromStr for PhpTypeName {
    type Err = PhpTypeError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        Self::new(input)
    }
}

pub type PhpClassLikeTypeName = PhpTypeName;

/// PHP type metadata without type-checker behavior.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum PhpType {
    Scalar(PhpScalarType),
    Named(PhpTypeName),
    Nullable(Box<PhpType>),
    Union(Vec<PhpType>),
    Intersection(Vec<PhpType>),
    Mixed,
    Never,
    Void,
    Callable,
    Iterable,
    Object,
}

impl PhpType {
    pub const fn scalar(kind: PhpScalarType) -> Self {
        Self::Scalar(kind)
    }

    pub const fn named(name: PhpTypeName) -> Self {
        Self::Named(name)
    }

    pub fn nullable(inner: Self) -> Result<Self, PhpTypeError> {
        if matches!(inner, Self::Void | Self::Never) {
            Err(PhpTypeError::InvalidComposite)
        } else {
            Ok(Self::Nullable(Box::new(inner)))
        }
    }

    pub fn union(types: Vec<Self>) -> Result<Self, PhpTypeError> {
        if types.len() < 2 {
            Err(PhpTypeError::TooFewTypes)
        } else {
            Ok(Self::Union(types))
        }
    }

    pub fn intersection(types: Vec<Self>) -> Result<Self, PhpTypeError> {
        if types.len() < 2 {
            Err(PhpTypeError::TooFewTypes)
        } else {
            Ok(Self::Intersection(types))
        }
    }

    pub const fn kind(&self) -> PhpTypeKind {
        match self {
            Self::Scalar(_) => PhpTypeKind::Scalar,
            Self::Named(_) => PhpTypeKind::ClassLike,
            Self::Nullable(_) => PhpTypeKind::Nullable,
            Self::Union(_) => PhpTypeKind::Union,
            Self::Intersection(_) => PhpTypeKind::Intersection,
            Self::Mixed => PhpTypeKind::Mixed,
            Self::Never => PhpTypeKind::Never,
            Self::Void => PhpTypeKind::Void,
            Self::Callable => PhpTypeKind::Callable,
            Self::Iterable => PhpTypeKind::Iterable,
            Self::Object => PhpTypeKind::Object,
        }
    }
}

impl fmt::Display for PhpType {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Scalar(kind) => formatter.write_str(kind.as_str()),
            Self::Named(name) => formatter.write_str(name.as_str()),
            Self::Nullable(inner) => write!(formatter, "?{inner}"),
            Self::Union(types) => write_joined_types(formatter, types, "|"),
            Self::Intersection(types) => write_joined_types(formatter, types, "&"),
            Self::Mixed => formatter.write_str("mixed"),
            Self::Never => formatter.write_str("never"),
            Self::Void => formatter.write_str("void"),
            Self::Callable => formatter.write_str("callable"),
            Self::Iterable => formatter.write_str("iterable"),
            Self::Object => formatter.write_str("object"),
        }
    }
}

/// Error returned when PHP type metadata is invalid.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PhpTypeError {
    Empty,
    EmptySegment,
    InvalidName,
    InvalidComposite,
    TooFewTypes,
    UnknownLabel,
}

impl fmt::Display for PhpTypeError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => formatter.write_str("PHP type metadata cannot be empty"),
            Self::EmptySegment => {
                formatter.write_str("PHP type name cannot contain empty segments")
            },
            Self::InvalidName => formatter.write_str("PHP type name has an invalid shape"),
            Self::InvalidComposite => formatter.write_str("PHP type composite is invalid"),
            Self::TooFewTypes => formatter.write_str("PHP composite type needs at least two types"),
            Self::UnknownLabel => formatter.write_str("unknown PHP type metadata label"),
        }
    }
}

impl Error for PhpTypeError {}

fn write_joined_types(
    formatter: &mut fmt::Formatter<'_>,
    types: &[PhpType],
    separator: &str,
) -> fmt::Result {
    for (index, value) in types.iter().enumerate() {
        if index > 0 {
            formatter.write_str(separator)?;
        }
        write!(formatter, "{value}")?;
    }
    Ok(())
}

fn validate_type_name(input: &str) -> Result<(), PhpTypeError> {
    if input.is_empty() {
        return Err(PhpTypeError::Empty);
    }
    for segment in input.split('\\') {
        if segment.is_empty() {
            return Err(PhpTypeError::EmptySegment);
        }
        let mut characters = segment.chars();
        let Some(first) = characters.next() else {
            return Err(PhpTypeError::EmptySegment);
        };
        if !(first == '_' || first.is_ascii_alphabetic())
            || !characters.all(|character| character == '_' || character.is_ascii_alphanumeric())
        {
            return Err(PhpTypeError::InvalidName);
        }
    }
    Ok(())
}

fn normalized_label(input: &str) -> Result<String, PhpTypeError> {
    let trimmed = input.trim();
    if trimmed.is_empty() {
        Err(PhpTypeError::Empty)
    } else {
        Ok(trimmed.to_ascii_lowercase())
    }
}

#[cfg(test)]
mod tests {
    use super::{PhpScalarType, PhpType, PhpTypeError, PhpTypeName};

    #[test]
    fn builds_union_and_nullable_types() -> Result<(), PhpTypeError> {
        let dto = PhpType::named(PhpTypeName::new("App\\Dto\\UserData")?);
        let union = PhpType::union(vec![PhpType::scalar(PhpScalarType::String), dto])?;
        let nullable = PhpType::nullable(PhpType::scalar(PhpScalarType::Int))?;

        assert_eq!(union.to_string(), "string|App\\Dto\\UserData");
        assert_eq!(nullable.to_string(), "?int");
        assert_eq!(union.kind().as_str(), "union");
        Ok(())
    }
}
