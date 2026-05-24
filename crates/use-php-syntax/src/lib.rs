#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

use core::{fmt, str::FromStr};
use std::error::Error;

macro_rules! syntax_enum {
    ($name:ident { $($variant:ident => $label:literal),+ $(,)? }) => {
        #[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
        pub enum $name {
            $($variant),+
        }

        impl $name {
            pub const ALL: &'static [Self] = &[$(Self::$variant),+];

            pub const fn as_str(self) -> &'static str {
                match self {
                    $(Self::$variant => $label),+
                }
            }
        }

        impl fmt::Display for $name {
            fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                formatter.write_str(self.as_str())
            }
        }

        impl FromStr for $name {
            type Err = PhpSyntaxError;

            fn from_str(input: &str) -> Result<Self, Self::Err> {
                match normalized_label(input)?.as_str() {
                    $($label => Ok(Self::$variant),)+
                    _ => Err(PhpSyntaxError::UnknownLabel),
                }
            }
        }
    };
}

syntax_enum!(PhpKeyword {
    Abstract => "abstract",
    And => "and",
    Array => "array",
    As => "as",
    Break => "break",
    Callable => "callable",
    Case => "case",
    Catch => "catch",
    Class => "class",
    Clone => "clone",
    Const => "const",
    Continue => "continue",
    Declare => "declare",
    Default => "default",
    Do => "do",
    Echo => "echo",
    Else => "else",
    Enum => "enum",
    Extends => "extends",
    Final => "final",
    Finally => "finally",
    Fn => "fn",
    For => "for",
    Foreach => "foreach",
    Function => "function",
    Global => "global",
    If => "if",
    Implements => "implements",
    Interface => "interface",
    Match => "match",
    Namespace => "namespace",
    New => "new",
    Or => "or",
    Private => "private",
    Protected => "protected",
    Public => "public",
    Readonly => "readonly",
    Return => "return",
    Static => "static",
    Switch => "switch",
    Throw => "throw",
    Trait => "trait",
    Try => "try",
    Use => "use",
    While => "while",
    Yield => "yield",
});

syntax_enum!(PhpVisibility {
    Public => "public",
    Protected => "protected",
    Private => "private",
});

syntax_enum!(PhpDeclarationKind {
    Class => "class",
    Interface => "interface",
    Trait => "trait",
    Enum => "enum",
    Function => "function",
    Method => "method",
    Property => "property",
    Constant => "constant",
});

syntax_enum!(PhpModifier {
    Static => "static",
    Final => "final",
    Abstract => "abstract",
    Readonly => "readonly",
});

syntax_enum!(PhpControlFlowLabel {
    If => "if",
    Else => "else",
    ElseIf => "elseif",
    For => "for",
    Foreach => "foreach",
    While => "while",
    DoWhile => "do-while",
    Switch => "switch",
    Match => "match",
    Try => "try",
    Catch => "catch",
    Finally => "finally",
    Break => "break",
    Continue => "continue",
    Return => "return",
    Throw => "throw",
    Yield => "yield",
});

/// Error returned when PHP syntax metadata cannot be parsed.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PhpSyntaxError {
    Empty,
    UnknownLabel,
}

impl fmt::Display for PhpSyntaxError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => formatter.write_str("PHP syntax metadata cannot be empty"),
            Self::UnknownLabel => formatter.write_str("unknown PHP syntax metadata label"),
        }
    }
}

impl Error for PhpSyntaxError {}

pub fn is_php_keyword(input: &str) -> bool {
    input.parse::<PhpKeyword>().is_ok()
}

pub fn is_php_modifier(input: &str) -> bool {
    input.parse::<PhpModifier>().is_ok()
}

fn normalized_label(input: &str) -> Result<String, PhpSyntaxError> {
    let trimmed = input.trim();
    if trimmed.is_empty() {
        Err(PhpSyntaxError::Empty)
    } else {
        Ok(trimmed.to_ascii_lowercase().replace(['_', ' '], "-"))
    }
}

#[cfg(test)]
mod tests {
    use super::{
        PhpControlFlowLabel, PhpDeclarationKind, PhpKeyword, PhpModifier, PhpSyntaxError,
        is_php_keyword, is_php_modifier,
    };

    #[test]
    fn parses_common_syntax_labels() -> Result<(), PhpSyntaxError> {
        assert!(is_php_keyword("readonly"));
        assert!(is_php_modifier("final"));
        assert_eq!("class".parse::<PhpKeyword>()?, PhpKeyword::Class);
        assert_eq!(
            "trait".parse::<PhpDeclarationKind>()?,
            PhpDeclarationKind::Trait
        );
        assert_eq!(
            "do while".parse::<PhpControlFlowLabel>()?,
            PhpControlFlowLabel::DoWhile
        );
        Ok(())
    }

    #[test]
    fn display_returns_source_labels() {
        assert_eq!(PhpModifier::Readonly.to_string(), "readonly");
    }
}
