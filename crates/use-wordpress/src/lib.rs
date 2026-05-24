#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

use core::{fmt, str::FromStr};
use std::error::Error;

macro_rules! wp_text_newtype {
    ($name:ident) => {
        #[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
        pub struct $name(String);

        impl $name {
            pub fn new(input: &str) -> Result<Self, WordPressError> {
                let trimmed = input.trim();
                if trimmed.is_empty() {
                    Err(WordPressError::Empty)
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
            type Err = WordPressError;

            fn from_str(input: &str) -> Result<Self, Self::Err> {
                Self::new(input)
            }
        }
    };
}

wp_text_newtype!(WordPressText);
wp_text_newtype!(WordPressPluginHeaderName);
wp_text_newtype!(WordPressThemeFieldName);
wp_text_newtype!(WordPressPostTypeSlug);
wp_text_newtype!(WordPressTaxonomySlug);
wp_text_newtype!(WordPressCapability);
wp_text_newtype!(WordPressRestNamespace);
wp_text_newtype!(WordPressRestRoute);

/// WordPress plugin header metadata.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WordPressPluginHeader {
    name: WordPressPluginHeaderName,
    value: WordPressText,
}

impl WordPressPluginHeader {
    pub fn new(name: WordPressPluginHeaderName, value: WordPressText) -> Self {
        Self { name, value }
    }

    pub const fn name(&self) -> &WordPressPluginHeaderName {
        &self.name
    }

    pub const fn value(&self) -> &WordPressText {
        &self.value
    }
}

/// WordPress theme metadata field.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WordPressThemeField {
    name: WordPressThemeFieldName,
    value: WordPressText,
}

impl WordPressThemeField {
    pub fn new(name: WordPressThemeFieldName, value: WordPressText) -> Self {
        Self { name, value }
    }

    pub const fn name(&self) -> &WordPressThemeFieldName {
        &self.name
    }

    pub const fn value(&self) -> &WordPressText {
        &self.value
    }
}

/// WordPress primitive metadata kind.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum WordPressMetadataKind {
    PluginHeader,
    ThemeField,
    PostType,
    Taxonomy,
    Capability,
    RestRoute,
}

impl WordPressMetadataKind {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::PluginHeader => "plugin-header",
            Self::ThemeField => "theme-field",
            Self::PostType => "post-type",
            Self::Taxonomy => "taxonomy",
            Self::Capability => "capability",
            Self::RestRoute => "rest-route",
        }
    }
}

/// Error returned when WordPress metadata is invalid.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WordPressError {
    Empty,
}

impl fmt::Display for WordPressError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("WordPress metadata cannot be empty")
    }
}

impl Error for WordPressError {}

#[cfg(test)]
mod tests {
    use super::{
        WordPressCapability, WordPressError, WordPressMetadataKind, WordPressPluginHeader,
        WordPressPluginHeaderName, WordPressPostTypeSlug, WordPressText,
    };

    #[test]
    fn builds_wordpress_metadata() -> Result<(), WordPressError> {
        let header = WordPressPluginHeader::new(
            WordPressPluginHeaderName::new("Plugin Name")?,
            WordPressText::new("Example Tools")?,
        );
        let post_type = WordPressPostTypeSlug::new("book")?;
        let capability = WordPressCapability::new("edit_posts")?;

        assert_eq!(header.name().as_str(), "Plugin Name");
        assert_eq!(post_type.as_str(), "book");
        assert_eq!(capability.as_str(), "edit_posts");
        assert_eq!(WordPressMetadataKind::RestRoute.as_str(), "rest-route");
        Ok(())
    }
}
