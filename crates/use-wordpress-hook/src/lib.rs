#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

use core::{fmt, str::FromStr};
use std::error::Error;

/// WordPress hook name metadata.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct HookName(String);

impl HookName {
    pub fn new(input: &str) -> Result<Self, WordPressHookError> {
        let trimmed = input.trim();
        if trimmed.is_empty() {
            Err(WordPressHookError::Empty)
        } else {
            Ok(Self(trimmed.to_string()))
        }
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for HookName {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl FromStr for HookName {
    type Err = WordPressHookError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        Self::new(input)
    }
}

/// WordPress hook kind metadata.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum HookKind {
    Action,
    Filter,
}

impl HookKind {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Action => "action",
            Self::Filter => "filter",
        }
    }
}

/// WordPress hook priority metadata.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct HookPriority(u16);

impl HookPriority {
    pub const DEFAULT: Self = Self(10);

    pub const fn new(value: u16) -> Self {
        Self(value)
    }

    pub const fn get(self) -> u16 {
        self.0
    }
}

/// WordPress callback metadata.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct HookCallbackMetadata {
    name: String,
    accepted_args: u8,
}

impl HookCallbackMetadata {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.trim().to_string(),
            accepted_args: 1,
        }
    }

    pub const fn with_accepted_args(mut self, accepted_args: u8) -> Self {
        self.accepted_args = accepted_args;
        self
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub const fn accepted_args(&self) -> u8 {
        self.accepted_args
    }
}

/// WordPress hook reference metadata.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct HookReference {
    name: HookName,
    kind: HookKind,
    priority: HookPriority,
    callback: Option<HookCallbackMetadata>,
}

impl HookReference {
    pub const fn new(name: HookName, kind: HookKind) -> Self {
        Self {
            name,
            kind,
            priority: HookPriority::DEFAULT,
            callback: None,
        }
    }

    pub const fn with_priority(mut self, priority: HookPriority) -> Self {
        self.priority = priority;
        self
    }

    pub fn with_callback(mut self, callback: HookCallbackMetadata) -> Self {
        self.callback = Some(callback);
        self
    }

    pub const fn name(&self) -> &HookName {
        &self.name
    }

    pub const fn kind(&self) -> HookKind {
        self.kind
    }

    pub const fn priority(&self) -> HookPriority {
        self.priority
    }

    pub const fn callback(&self) -> Option<&HookCallbackMetadata> {
        self.callback.as_ref()
    }
}

/// Error returned when WordPress hook metadata is invalid.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WordPressHookError {
    Empty,
}

impl fmt::Display for WordPressHookError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("WordPress hook metadata cannot be empty")
    }
}

impl Error for WordPressHookError {}

#[cfg(test)]
mod tests {
    use super::{
        HookCallbackMetadata, HookKind, HookName, HookPriority, HookReference, WordPressHookError,
    };

    #[test]
    fn builds_hook_reference() -> Result<(), WordPressHookError> {
        let hook = HookReference::new(HookName::new("init")?, HookKind::Action)
            .with_priority(HookPriority::new(20))
            .with_callback(HookCallbackMetadata::new("register_books").with_accepted_args(2));

        assert_eq!(hook.name().as_str(), "init");
        assert_eq!(hook.priority().get(), 20);
        assert_eq!(hook.callback().expect("callback").accepted_args(), 2);
        Ok(())
    }
}
