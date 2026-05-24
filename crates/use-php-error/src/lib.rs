#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

use core::{fmt, str::FromStr};
use std::error::Error;

macro_rules! diagnostic_text_newtype {
    ($name:ident) => {
        #[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
        pub struct $name(String);

        impl $name {
            pub fn new(input: &str) -> Result<Self, PhpDiagnosticError> {
                let trimmed = input.trim();
                if trimmed.is_empty() {
                    Err(PhpDiagnosticError::Empty)
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
    };
}

diagnostic_text_newtype!(DiagnosticMessage);
diagnostic_text_newtype!(DiagnosticSource);

/// PHP error level metadata.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum PhpErrorLevel {
    Error,
    Warning,
    Parse,
    Notice,
    CoreError,
    CoreWarning,
    CompileError,
    CompileWarning,
    UserError,
    UserWarning,
    UserNotice,
    Deprecated,
    UserDeprecated,
    RecoverableError,
}

impl PhpErrorLevel {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Error => "E_ERROR",
            Self::Warning => "E_WARNING",
            Self::Parse => "E_PARSE",
            Self::Notice => "E_NOTICE",
            Self::CoreError => "E_CORE_ERROR",
            Self::CoreWarning => "E_CORE_WARNING",
            Self::CompileError => "E_COMPILE_ERROR",
            Self::CompileWarning => "E_COMPILE_WARNING",
            Self::UserError => "E_USER_ERROR",
            Self::UserWarning => "E_USER_WARNING",
            Self::UserNotice => "E_USER_NOTICE",
            Self::Deprecated => "E_DEPRECATED",
            Self::UserDeprecated => "E_USER_DEPRECATED",
            Self::RecoverableError => "E_RECOVERABLE_ERROR",
        }
    }

    pub const fn severity(self) -> PhpSeverity {
        match self {
            Self::Error
            | Self::Parse
            | Self::CoreError
            | Self::CompileError
            | Self::UserError
            | Self::RecoverableError => PhpSeverity::Error,
            Self::Warning | Self::CoreWarning | Self::CompileWarning | Self::UserWarning => {
                PhpSeverity::Warning
            },
            Self::Deprecated | Self::UserDeprecated => PhpSeverity::Deprecated,
            Self::Notice | Self::UserNotice => PhpSeverity::Info,
        }
    }
}

impl fmt::Display for PhpErrorLevel {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

/// Severity metadata for PHP diagnostics.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum PhpSeverity {
    Error,
    Warning,
    Info,
    Deprecated,
}

impl PhpSeverity {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Error => "error",
            Self::Warning => "warning",
            Self::Info => "info",
            Self::Deprecated => "deprecated",
        }
    }
}

impl fmt::Display for PhpSeverity {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

/// Broad PHP diagnostic category metadata.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum PhpErrorKind {
    Runtime,
    Parse,
    Compile,
    Core,
    User,
    Deprecation,
    Unknown,
}

impl PhpErrorKind {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Runtime => "runtime",
            Self::Parse => "parse",
            Self::Compile => "compile",
            Self::Core => "core",
            Self::User => "user",
            Self::Deprecation => "deprecation",
            Self::Unknown => "unknown",
        }
    }
}

impl fmt::Display for PhpErrorKind {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl FromStr for PhpErrorKind {
    type Err = PhpDiagnosticError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input.trim().to_ascii_lowercase().as_str() {
            "runtime" => Ok(Self::Runtime),
            "parse" => Ok(Self::Parse),
            "compile" => Ok(Self::Compile),
            "core" => Ok(Self::Core),
            "user" => Ok(Self::User),
            "deprecation" | "deprecated" => Ok(Self::Deprecation),
            "unknown" => Ok(Self::Unknown),
            "" => Err(PhpDiagnosticError::Empty),
            _ => Err(PhpDiagnosticError::UnknownLabel),
        }
    }
}

/// PHP diagnostic metadata.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PhpDiagnostic {
    kind: PhpErrorKind,
    severity: PhpSeverity,
    message: DiagnosticMessage,
    source: Option<DiagnosticSource>,
}

impl PhpDiagnostic {
    pub const fn new(
        kind: PhpErrorKind,
        severity: PhpSeverity,
        message: DiagnosticMessage,
    ) -> Self {
        Self {
            kind,
            severity,
            message,
            source: None,
        }
    }

    pub fn with_source(mut self, source: DiagnosticSource) -> Self {
        self.source = Some(source);
        self
    }

    pub const fn kind(&self) -> PhpErrorKind {
        self.kind
    }

    pub const fn severity(&self) -> PhpSeverity {
        self.severity
    }

    pub const fn message(&self) -> &DiagnosticMessage {
        &self.message
    }

    pub const fn source(&self) -> Option<&DiagnosticSource> {
        self.source.as_ref()
    }
}

/// Error returned when PHP diagnostic metadata is invalid.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PhpDiagnosticError {
    Empty,
    UnknownLabel,
}

impl fmt::Display for PhpDiagnosticError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => formatter.write_str("PHP diagnostic metadata cannot be empty"),
            Self::UnknownLabel => formatter.write_str("unknown PHP diagnostic metadata label"),
        }
    }
}

impl Error for PhpDiagnosticError {}

#[cfg(test)]
mod tests {
    use super::{
        DiagnosticMessage, DiagnosticSource, PhpDiagnostic, PhpDiagnosticError, PhpErrorKind,
        PhpErrorLevel, PhpSeverity,
    };

    #[test]
    fn maps_error_level_to_severity() {
        assert_eq!(PhpErrorLevel::UserWarning.severity(), PhpSeverity::Warning);
        assert_eq!(
            PhpErrorLevel::Deprecated.severity(),
            PhpSeverity::Deprecated
        );
    }

    #[test]
    fn builds_diagnostic_metadata() -> Result<(), PhpDiagnosticError> {
        let diagnostic = PhpDiagnostic::new(
            PhpErrorKind::Runtime,
            PhpSeverity::Error,
            DiagnosticMessage::new("Undefined variable")?,
        )
        .with_source(DiagnosticSource::new("example.php:10")?);

        assert_eq!(diagnostic.severity(), PhpSeverity::Error);
        assert_eq!(
            diagnostic.source().expect("source").as_str(),
            "example.php:10"
        );
        Ok(())
    }
}
