#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

use core::{fmt, str::FromStr};
use std::error::Error;

macro_rules! autoload_text_newtype {
    ($name:ident) => {
        #[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
        pub struct $name(String);

        impl $name {
            pub fn new(input: &str) -> Result<Self, PhpAutoloadError> {
                let trimmed = input.trim();
                if trimmed.is_empty() {
                    Err(PhpAutoloadError::Empty)
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

autoload_text_newtype!(AutoloadPath);

/// PSR-4 namespace prefix metadata.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Psr4Prefix(String);

impl Psr4Prefix {
    pub fn new(input: &str) -> Result<Self, PhpAutoloadError> {
        let trimmed = input.trim();
        if trimmed.is_empty() {
            return Err(PhpAutoloadError::Empty);
        }
        if !trimmed.ends_with('\\') {
            return Err(PhpAutoloadError::InvalidPrefix);
        }
        Ok(Self(trimmed.to_string()))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for Psr4Prefix {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl FromStr for Psr4Prefix {
    type Err = PhpAutoloadError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        Self::new(input)
    }
}

/// PSR-4 prefix-to-path mapping metadata.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Psr4Mapping {
    prefix: Psr4Prefix,
    paths: Vec<AutoloadPath>,
}

impl Psr4Mapping {
    pub fn new(prefix: Psr4Prefix) -> Self {
        Self {
            prefix,
            paths: Vec::new(),
        }
    }

    pub fn with_path(mut self, path: AutoloadPath) -> Self {
        self.paths.push(path);
        self
    }

    pub const fn prefix(&self) -> &Psr4Prefix {
        &self.prefix
    }

    pub fn paths(&self) -> &[AutoloadPath] {
        &self.paths
    }
}

/// Classmap entry metadata.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ClassmapEntry {
    class_name: String,
    path: AutoloadPath,
}

impl ClassmapEntry {
    pub fn new(class_name: &str, path: AutoloadPath) -> Self {
        Self {
            class_name: class_name.trim().to_string(),
            path,
        }
    }

    pub fn class_name(&self) -> &str {
        &self.class_name
    }

    pub const fn path(&self) -> &AutoloadPath {
        &self.path
    }
}

/// Composer-style files autoload entry metadata.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FilesAutoloadEntry {
    path: AutoloadPath,
}

impl FilesAutoloadEntry {
    pub const fn new(path: AutoloadPath) -> Self {
        Self { path }
    }

    pub const fn path(&self) -> &AutoloadPath {
        &self.path
    }
}

/// PHP autoload strategy metadata.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum AutoloadStrategy {
    Psr4,
    Classmap,
    Files,
    IncludePath,
}

impl AutoloadStrategy {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Psr4 => "psr-4",
            Self::Classmap => "classmap",
            Self::Files => "files",
            Self::IncludePath => "include-path",
        }
    }
}

impl fmt::Display for AutoloadStrategy {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

/// Lightweight autoload configuration metadata.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct AutoloadConfig {
    psr4: Vec<Psr4Mapping>,
    classmap: Vec<ClassmapEntry>,
    files: Vec<FilesAutoloadEntry>,
}

impl AutoloadConfig {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_psr4(mut self, mapping: Psr4Mapping) -> Self {
        self.psr4.push(mapping);
        self
    }

    pub fn with_classmap(mut self, entry: ClassmapEntry) -> Self {
        self.classmap.push(entry);
        self
    }

    pub fn with_file(mut self, entry: FilesAutoloadEntry) -> Self {
        self.files.push(entry);
        self
    }

    pub fn psr4(&self) -> &[Psr4Mapping] {
        &self.psr4
    }

    pub fn classmap(&self) -> &[ClassmapEntry] {
        &self.classmap
    }

    pub fn files(&self) -> &[FilesAutoloadEntry] {
        &self.files
    }
}

/// Error returned when PHP autoload metadata is invalid.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PhpAutoloadError {
    Empty,
    InvalidPrefix,
}

impl fmt::Display for PhpAutoloadError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => formatter.write_str("PHP autoload metadata cannot be empty"),
            Self::InvalidPrefix => {
                formatter.write_str("PSR-4 prefixes must end with a namespace separator")
            },
        }
    }
}

impl Error for PhpAutoloadError {}

#[cfg(test)]
mod tests {
    use super::{
        AutoloadConfig, AutoloadPath, ClassmapEntry, FilesAutoloadEntry, PhpAutoloadError,
        Psr4Mapping, Psr4Prefix,
    };

    #[test]
    fn builds_autoload_metadata() -> Result<(), PhpAutoloadError> {
        let mapping =
            Psr4Mapping::new(Psr4Prefix::new("App\\")?).with_path(AutoloadPath::new("src/")?);
        let classmap = ClassmapEntry::new(
            "Legacy_Class",
            AutoloadPath::new("legacy/Legacy_Class.php")?,
        );
        let config = AutoloadConfig::new()
            .with_psr4(mapping)
            .with_classmap(classmap)
            .with_file(FilesAutoloadEntry::new(AutoloadPath::new("bootstrap.php")?));

        assert_eq!(config.psr4()[0].prefix().as_str(), "App\\");
        assert_eq!(config.classmap()[0].class_name(), "Legacy_Class");
        assert_eq!(config.files()[0].path().as_str(), "bootstrap.php");
        Ok(())
    }
}
