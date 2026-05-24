#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

use core::{fmt, str::FromStr};
use std::{collections::BTreeMap, error::Error};

macro_rules! composer_text_newtype {
    ($name:ident) => {
        #[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
        pub struct $name(String);

        impl $name {
            pub fn new(input: &str) -> Result<Self, ComposerJsonError> {
                let trimmed = input.trim();
                if trimmed.is_empty() {
                    Err(ComposerJsonError::Empty)
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
            type Err = ComposerJsonError;

            fn from_str(input: &str) -> Result<Self, Self::Err> {
                Self::new(input)
            }
        }
    };
}

composer_text_newtype!(ComposerVendorName);
composer_text_newtype!(ComposerPackageShortName);
composer_text_newtype!(ComposerRequirement);
composer_text_newtype!(ComposerScriptName);
composer_text_newtype!(ComposerScript);
composer_text_newtype!(ComposerRepositoryUrl);

/// Composer package name metadata in `vendor/package` form.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct ComposerPackageName {
    vendor: String,
    package: String,
}

impl ComposerPackageName {
    pub fn new(input: &str) -> Result<Self, ComposerJsonError> {
        let trimmed = input.trim();
        let Some((vendor, package)) = trimmed.split_once('/') else {
            return Err(ComposerJsonError::InvalidPackageName);
        };
        if vendor.is_empty() || package.is_empty() || package.contains('/') {
            return Err(ComposerJsonError::InvalidPackageName);
        }
        if trimmed.chars().any(char::is_whitespace) {
            return Err(ComposerJsonError::ContainsWhitespace);
        }
        Ok(Self {
            vendor: vendor.to_string(),
            package: package.to_string(),
        })
    }

    pub fn vendor(&self) -> &str {
        &self.vendor
    }

    pub fn package(&self) -> &str {
        &self.package
    }
}

impl fmt::Display for ComposerPackageName {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}/{}", self.vendor, self.package)
    }
}

impl FromStr for ComposerPackageName {
    type Err = ComposerJsonError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        Self::new(input)
    }
}

/// Composer repository kind metadata.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum ComposerRepositoryKind {
    Composer,
    Vcs,
    Path,
    Artifact,
    Package,
}

impl ComposerRepositoryKind {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Composer => "composer",
            Self::Vcs => "vcs",
            Self::Path => "path",
            Self::Artifact => "artifact",
            Self::Package => "package",
        }
    }
}

/// Composer stability label metadata.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum ComposerStability {
    Dev,
    Alpha,
    Beta,
    Rc,
    Stable,
}

impl ComposerStability {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Dev => "dev",
            Self::Alpha => "alpha",
            Self::Beta => "beta",
            Self::Rc => "RC",
            Self::Stable => "stable",
        }
    }
}

/// Composer package type metadata.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum ComposerPackageType {
    Library,
    Project,
    Metapackage,
    ComposerPlugin,
    Other,
}

impl ComposerPackageType {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Library => "library",
            Self::Project => "project",
            Self::Metapackage => "metapackage",
            Self::ComposerPlugin => "composer-plugin",
            Self::Other => "other",
        }
    }
}

/// Composer repository metadata.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ComposerRepository {
    kind: ComposerRepositoryKind,
    url: Option<ComposerRepositoryUrl>,
}

impl ComposerRepository {
    pub const fn new(kind: ComposerRepositoryKind) -> Self {
        Self { kind, url: None }
    }

    pub fn with_url(mut self, url: ComposerRepositoryUrl) -> Self {
        self.url = Some(url);
        self
    }

    pub const fn kind(&self) -> ComposerRepositoryKind {
        self.kind
    }

    pub const fn url(&self) -> Option<&ComposerRepositoryUrl> {
        self.url.as_ref()
    }
}

/// Composer autoload metadata without resolving paths.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct ComposerAutoloadConfig {
    psr4: BTreeMap<String, Vec<String>>,
    classmap: Vec<String>,
    files: Vec<String>,
}

impl ComposerAutoloadConfig {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_psr4(mut self, prefix: &str, path: &str) -> Self {
        self.psr4
            .entry(prefix.to_string())
            .or_default()
            .push(path.to_string());
        self
    }

    pub fn with_classmap(mut self, path: &str) -> Self {
        self.classmap.push(path.to_string());
        self
    }

    pub fn with_file(mut self, path: &str) -> Self {
        self.files.push(path.to_string());
        self
    }

    pub const fn psr4(&self) -> &BTreeMap<String, Vec<String>> {
        &self.psr4
    }

    pub fn classmap(&self) -> &[String] {
        &self.classmap
    }

    pub fn files(&self) -> &[String] {
        &self.files
    }
}

/// Partial practical Composer JSON metadata.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct ComposerJson {
    name: Option<ComposerPackageName>,
    package_type: Option<ComposerPackageType>,
    minimum_stability: Option<ComposerStability>,
    requirements: BTreeMap<String, ComposerRequirement>,
    dev_requirements: BTreeMap<String, ComposerRequirement>,
    scripts: BTreeMap<ComposerScriptName, ComposerScript>,
    repositories: Vec<ComposerRepository>,
    autoload: Option<ComposerAutoloadConfig>,
}

impl ComposerJson {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_name(mut self, name: ComposerPackageName) -> Self {
        self.name = Some(name);
        self
    }

    pub const fn with_package_type(mut self, package_type: ComposerPackageType) -> Self {
        self.package_type = Some(package_type);
        self
    }

    pub const fn with_minimum_stability(mut self, stability: ComposerStability) -> Self {
        self.minimum_stability = Some(stability);
        self
    }

    pub fn with_requirement(mut self, name: &str, requirement: ComposerRequirement) -> Self {
        self.requirements.insert(name.to_string(), requirement);
        self
    }

    pub fn with_script(mut self, name: ComposerScriptName, script: ComposerScript) -> Self {
        self.scripts.insert(name, script);
        self
    }

    pub fn with_repository(mut self, repository: ComposerRepository) -> Self {
        self.repositories.push(repository);
        self
    }

    pub fn with_autoload(mut self, autoload: ComposerAutoloadConfig) -> Self {
        self.autoload = Some(autoload);
        self
    }

    pub const fn name(&self) -> Option<&ComposerPackageName> {
        self.name.as_ref()
    }

    pub const fn package_type(&self) -> Option<ComposerPackageType> {
        self.package_type
    }

    pub const fn minimum_stability(&self) -> Option<ComposerStability> {
        self.minimum_stability
    }

    pub const fn requirements(&self) -> &BTreeMap<String, ComposerRequirement> {
        &self.requirements
    }

    pub const fn dev_requirements(&self) -> &BTreeMap<String, ComposerRequirement> {
        &self.dev_requirements
    }

    pub const fn autoload(&self) -> Option<&ComposerAutoloadConfig> {
        self.autoload.as_ref()
    }
}

/// Error returned when Composer metadata is invalid.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ComposerJsonError {
    Empty,
    ContainsWhitespace,
    InvalidPackageName,
}

impl fmt::Display for ComposerJsonError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => formatter.write_str("Composer metadata cannot be empty"),
            Self::ContainsWhitespace => {
                formatter.write_str("Composer package name cannot contain whitespace")
            },
            Self::InvalidPackageName => {
                formatter.write_str("Composer package name must look like vendor/package")
            },
        }
    }
}

impl Error for ComposerJsonError {}

#[cfg(test)]
mod tests {
    use super::{
        ComposerAutoloadConfig, ComposerJson, ComposerJsonError, ComposerPackageName,
        ComposerPackageType, ComposerRequirement,
    };

    #[test]
    fn builds_composer_json_metadata() -> Result<(), ComposerJsonError> {
        let package = ComposerJson::new()
            .with_name(ComposerPackageName::new("acme/demo")?)
            .with_package_type(ComposerPackageType::Library)
            .with_requirement("php", ComposerRequirement::new("^8.2")?)
            .with_autoload(ComposerAutoloadConfig::new().with_psr4("Acme\\Demo\\", "src/"));

        assert_eq!(package.name().expect("name").vendor(), "acme");
        assert!(package.requirements().contains_key("php"));
        assert!(
            package
                .autoload()
                .expect("autoload")
                .psr4()
                .contains_key("Acme\\Demo\\")
        );
        Ok(())
    }
}
