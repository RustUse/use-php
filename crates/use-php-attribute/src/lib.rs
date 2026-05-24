#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

use core::{fmt, str::FromStr};
use std::error::Error;

macro_rules! text_newtype {
    ($name:ident) => {
        #[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
        pub struct $name(String);

        impl $name {
            pub fn new(input: &str) -> Result<Self, PhpAttributeError> {
                let trimmed = input.trim();
                if trimmed.is_empty() {
                    Err(PhpAttributeError::Empty)
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
            type Err = PhpAttributeError;

            fn from_str(input: &str) -> Result<Self, Self::Err> {
                Self::new(input)
            }
        }
    };
}

text_newtype!(PhpAttributeName);
text_newtype!(PhpAttributeArgumentName);
text_newtype!(PhpAttributeArgumentValue);

/// PHP attribute target metadata.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum PhpAttributeTarget {
    Class,
    Function,
    Method,
    Property,
    ClassConstant,
    Parameter,
    EnumCase,
    All,
}

impl PhpAttributeTarget {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Class => "class",
            Self::Function => "function",
            Self::Method => "method",
            Self::Property => "property",
            Self::ClassConstant => "class-constant",
            Self::Parameter => "parameter",
            Self::EnumCase => "enum-case",
            Self::All => "all",
        }
    }
}

impl fmt::Display for PhpAttributeTarget {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

/// PHP attribute repeatability metadata.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum PhpAttributeRepeatability {
    Single,
    Repeatable,
}

impl PhpAttributeRepeatability {
    pub const fn is_repeatable(self) -> bool {
        matches!(self, Self::Repeatable)
    }
}

/// Simple PHP attribute argument metadata.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PhpAttributeArgument {
    name: Option<PhpAttributeArgumentName>,
    value: PhpAttributeArgumentValue,
}

impl PhpAttributeArgument {
    pub const fn positional(value: PhpAttributeArgumentValue) -> Self {
        Self { name: None, value }
    }

    pub const fn named(name: PhpAttributeArgumentName, value: PhpAttributeArgumentValue) -> Self {
        Self {
            name: Some(name),
            value,
        }
    }

    pub const fn name(&self) -> Option<&PhpAttributeArgumentName> {
        self.name.as_ref()
    }

    pub const fn value(&self) -> &PhpAttributeArgumentValue {
        &self.value
    }
}

/// PHP attribute reference metadata.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PhpAttributeReference {
    name: PhpAttributeName,
    targets: Vec<PhpAttributeTarget>,
    arguments: Vec<PhpAttributeArgument>,
    repeatability: PhpAttributeRepeatability,
}

impl PhpAttributeReference {
    pub fn new(name: PhpAttributeName) -> Self {
        Self {
            name,
            targets: Vec::new(),
            arguments: Vec::new(),
            repeatability: PhpAttributeRepeatability::Single,
        }
    }

    pub fn with_target(mut self, target: PhpAttributeTarget) -> Self {
        self.targets.push(target);
        self
    }

    pub fn with_argument(mut self, argument: PhpAttributeArgument) -> Self {
        self.arguments.push(argument);
        self
    }

    pub const fn with_repeatability(mut self, repeatability: PhpAttributeRepeatability) -> Self {
        self.repeatability = repeatability;
        self
    }

    pub const fn name(&self) -> &PhpAttributeName {
        &self.name
    }

    pub fn targets(&self) -> &[PhpAttributeTarget] {
        &self.targets
    }

    pub fn arguments(&self) -> &[PhpAttributeArgument] {
        &self.arguments
    }

    pub const fn repeatability(&self) -> PhpAttributeRepeatability {
        self.repeatability
    }
}

/// Error returned when PHP attribute metadata is invalid.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PhpAttributeError {
    Empty,
}

impl fmt::Display for PhpAttributeError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("PHP attribute metadata cannot be empty")
    }
}

impl Error for PhpAttributeError {}

#[cfg(test)]
mod tests {
    use super::{
        PhpAttributeArgument, PhpAttributeArgumentValue, PhpAttributeError, PhpAttributeName,
        PhpAttributeReference, PhpAttributeRepeatability, PhpAttributeTarget,
    };

    #[test]
    fn builds_attribute_reference() -> Result<(), PhpAttributeError> {
        let reference = PhpAttributeReference::new(PhpAttributeName::new("App\\Route")?)
            .with_target(PhpAttributeTarget::Method)
            .with_argument(PhpAttributeArgument::positional(
                PhpAttributeArgumentValue::new("/home")?,
            ))
            .with_repeatability(PhpAttributeRepeatability::Repeatable);

        assert_eq!(reference.name().as_str(), "App\\Route");
        assert_eq!(reference.targets(), &[PhpAttributeTarget::Method]);
        assert!(reference.repeatability().is_repeatable());
        Ok(())
    }
}
