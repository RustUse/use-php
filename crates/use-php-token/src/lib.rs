#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

use core::{fmt, str::FromStr};
use std::error::Error;

/// Broad PHP token category metadata.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum PhpTokenCategory {
    OpeningTag,
    ClosingTag,
    Keyword,
    Identifier,
    Variable,
    Literal,
    Operator,
    Delimiter,
    Comment,
    Whitespace,
    Unknown,
}

impl PhpTokenCategory {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::OpeningTag => "opening-tag",
            Self::ClosingTag => "closing-tag",
            Self::Keyword => "keyword",
            Self::Identifier => "identifier",
            Self::Variable => "variable",
            Self::Literal => "literal",
            Self::Operator => "operator",
            Self::Delimiter => "delimiter",
            Self::Comment => "comment",
            Self::Whitespace => "whitespace",
            Self::Unknown => "unknown",
        }
    }
}

impl fmt::Display for PhpTokenCategory {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl FromStr for PhpTokenCategory {
    type Err = PhpTokenError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match normalized_label(input)?.as_str() {
            "openingtag" => Ok(Self::OpeningTag),
            "closingtag" => Ok(Self::ClosingTag),
            "keyword" => Ok(Self::Keyword),
            "identifier" => Ok(Self::Identifier),
            "variable" => Ok(Self::Variable),
            "literal" => Ok(Self::Literal),
            "operator" => Ok(Self::Operator),
            "delimiter" => Ok(Self::Delimiter),
            "comment" => Ok(Self::Comment),
            "whitespace" => Ok(Self::Whitespace),
            "unknown" => Ok(Self::Unknown),
            _ => Err(PhpTokenError::UnknownLabel),
        }
    }
}

/// PHP delimiter label metadata.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum PhpDelimiter {
    OpenParen,
    CloseParen,
    OpenBrace,
    CloseBrace,
    OpenBracket,
    CloseBracket,
    Semicolon,
    Comma,
    Colon,
    DoubleColon,
    Arrow,
    NamespaceSeparator,
}

impl PhpDelimiter {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::OpenParen => "(",
            Self::CloseParen => ")",
            Self::OpenBrace => "{",
            Self::CloseBrace => "}",
            Self::OpenBracket => "[",
            Self::CloseBracket => "]",
            Self::Semicolon => ";",
            Self::Comma => ",",
            Self::Colon => ":",
            Self::DoubleColon => "::",
            Self::Arrow => "->",
            Self::NamespaceSeparator => "\\",
        }
    }
}

impl fmt::Display for PhpDelimiter {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

/// PHP operator label metadata.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum PhpOperator {
    Assign,
    Plus,
    Minus,
    Multiply,
    Divide,
    Modulo,
    Equal,
    Identical,
    NotEqual,
    NotIdentical,
    Spaceship,
    NullCoalesce,
    Elvis,
}

impl PhpOperator {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Assign => "=",
            Self::Plus => "+",
            Self::Minus => "-",
            Self::Multiply => "*",
            Self::Divide => "/",
            Self::Modulo => "%",
            Self::Equal => "==",
            Self::Identical => "===",
            Self::NotEqual => "!=",
            Self::NotIdentical => "!==",
            Self::Spaceship => "<=>",
            Self::NullCoalesce => "??",
            Self::Elvis => "?:",
        }
    }
}

impl fmt::Display for PhpOperator {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

/// PHP literal category metadata.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum PhpLiteralKind {
    String,
    Integer,
    Float,
    Boolean,
    Null,
    Array,
    Heredoc,
    Nowdoc,
}

impl PhpLiteralKind {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::String => "string",
            Self::Integer => "integer",
            Self::Float => "float",
            Self::Boolean => "boolean",
            Self::Null => "null",
            Self::Array => "array",
            Self::Heredoc => "heredoc",
            Self::Nowdoc => "nowdoc",
        }
    }
}

/// PHP comment category metadata.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum PhpCommentKind {
    Line,
    Block,
    Docblock,
    HashLine,
}

impl PhpCommentKind {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Line => "line",
            Self::Block => "block",
            Self::Docblock => "docblock",
            Self::HashLine => "hash-line",
        }
    }
}

/// Non-empty token text metadata.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct PhpTokenText(String);

impl PhpTokenText {
    pub fn new(input: &str) -> Result<Self, PhpTokenError> {
        let trimmed = input.trim();
        if trimmed.is_empty() {
            Err(PhpTokenError::Empty)
        } else {
            Ok(Self(trimmed.to_string()))
        }
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for PhpTokenText {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl FromStr for PhpTokenText {
    type Err = PhpTokenError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        Self::new(input)
    }
}

/// Byte span metadata for a token in source text.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct PhpTokenSpan {
    start: usize,
    end: usize,
}

impl PhpTokenSpan {
    pub const fn new(start: usize, end: usize) -> Result<Self, PhpTokenError> {
        if end < start {
            Err(PhpTokenError::InvalidSpan)
        } else {
            Ok(Self { start, end })
        }
    }

    pub const fn start(self) -> usize {
        self.start
    }

    pub const fn end(self) -> usize {
        self.end
    }

    pub const fn len(self) -> usize {
        self.end - self.start
    }

    pub const fn is_empty(self) -> bool {
        self.start == self.end
    }
}

/// Lightweight token metadata.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PhpToken {
    category: PhpTokenCategory,
    text: PhpTokenText,
    span: PhpTokenSpan,
}

impl PhpToken {
    pub const fn new(category: PhpTokenCategory, text: PhpTokenText, span: PhpTokenSpan) -> Self {
        Self {
            category,
            text,
            span,
        }
    }

    pub const fn category(&self) -> PhpTokenCategory {
        self.category
    }

    pub const fn text(&self) -> &PhpTokenText {
        &self.text
    }

    pub const fn span(&self) -> PhpTokenSpan {
        self.span
    }
}

/// Error returned when PHP token metadata is invalid.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PhpTokenError {
    Empty,
    InvalidSpan,
    UnknownLabel,
}

impl fmt::Display for PhpTokenError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => formatter.write_str("PHP token metadata cannot be empty"),
            Self::InvalidSpan => formatter.write_str("PHP token span end cannot precede start"),
            Self::UnknownLabel => formatter.write_str("unknown PHP token metadata label"),
        }
    }
}

impl Error for PhpTokenError {}

fn normalized_label(input: &str) -> Result<String, PhpTokenError> {
    let trimmed = input.trim();
    if trimmed.is_empty() {
        Err(PhpTokenError::Empty)
    } else {
        Ok(trimmed.to_ascii_lowercase().replace(['-', '_', ' '], ""))
    }
}

#[cfg(test)]
mod tests {
    use super::{
        PhpDelimiter, PhpOperator, PhpToken, PhpTokenCategory, PhpTokenError, PhpTokenSpan,
        PhpTokenText,
    };

    #[test]
    fn builds_token_metadata() -> Result<(), PhpTokenError> {
        let token = PhpToken::new(
            PhpTokenCategory::Variable,
            PhpTokenText::new(" $value ")?,
            PhpTokenSpan::new(4, 10)?,
        );

        assert_eq!(token.category(), PhpTokenCategory::Variable);
        assert_eq!(token.text().as_str(), "$value");
        assert_eq!(token.span().len(), 6);
        Ok(())
    }

    #[test]
    fn exposes_operator_and_delimiter_labels() {
        assert_eq!(PhpDelimiter::DoubleColon.to_string(), "::");
        assert_eq!(PhpOperator::Spaceship.to_string(), "<=>");
    }
}
