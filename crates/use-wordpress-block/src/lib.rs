#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

use core::{fmt, str::FromStr};
use std::error::Error;

macro_rules! block_text_newtype {
    ($name:ident) => {
        #[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
        pub struct $name(String);

        impl $name {
            pub fn new(input: &str) -> Result<Self, WordPressBlockError> {
                let trimmed = input.trim();
                if trimmed.is_empty() {
                    Err(WordPressBlockError::Empty)
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
            type Err = WordPressBlockError;

            fn from_str(input: &str) -> Result<Self, Self::Err> {
                Self::new(input)
            }
        }
    };
}

block_text_newtype!(WordPressBlockCategory);
block_text_newtype!(WordPressBlockAttributeName);
block_text_newtype!(WordPressBlockAssetPath);

/// WordPress block name metadata in `namespace/block` form.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct WordPressBlockName(String);

impl WordPressBlockName {
    pub fn new(input: &str) -> Result<Self, WordPressBlockError> {
        let trimmed = input.trim();
        let Some((namespace, block)) = trimmed.split_once('/') else {
            return Err(WordPressBlockError::InvalidBlockName);
        };
        if namespace.is_empty() || block.is_empty() || block.contains('/') {
            return Err(WordPressBlockError::InvalidBlockName);
        }
        Ok(Self(trimmed.to_string()))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for WordPressBlockName {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl FromStr for WordPressBlockName {
    type Err = WordPressBlockError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        Self::new(input)
    }
}

/// WordPress block attribute type metadata.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum WordPressBlockAttributeType {
    String,
    Boolean,
    Number,
    Integer,
    Object,
    Array,
}

impl WordPressBlockAttributeType {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::String => "string",
            Self::Boolean => "boolean",
            Self::Number => "number",
            Self::Integer => "integer",
            Self::Object => "object",
            Self::Array => "array",
        }
    }
}

/// WordPress block attribute metadata.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WordPressBlockAttribute {
    name: WordPressBlockAttributeName,
    attribute_type: WordPressBlockAttributeType,
}

impl WordPressBlockAttribute {
    pub const fn new(
        name: WordPressBlockAttributeName,
        attribute_type: WordPressBlockAttributeType,
    ) -> Self {
        Self {
            name,
            attribute_type,
        }
    }

    pub const fn name(&self) -> &WordPressBlockAttributeName {
        &self.name
    }

    pub const fn attribute_type(&self) -> WordPressBlockAttributeType {
        self.attribute_type
    }
}

/// WordPress block support metadata.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum WordPressBlockSupport {
    Align,
    Color,
    Typography,
    Spacing,
    Html,
    Multiple,
}

impl WordPressBlockSupport {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Align => "align",
            Self::Color => "color",
            Self::Typography => "typography",
            Self::Spacing => "spacing",
            Self::Html => "html",
            Self::Multiple => "multiple",
        }
    }
}

pub type WordPressBlockSupports = Vec<WordPressBlockSupport>;

/// WordPress `block.json`-oriented metadata.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WordPressBlockJson {
    name: WordPressBlockName,
    category: Option<WordPressBlockCategory>,
    attributes: Vec<WordPressBlockAttribute>,
    supports: WordPressBlockSupports,
    editor_script: Option<WordPressBlockAssetPath>,
    style: Option<WordPressBlockAssetPath>,
    render: Option<WordPressBlockAssetPath>,
}

impl WordPressBlockJson {
    pub fn new(name: WordPressBlockName) -> Self {
        Self {
            name,
            category: None,
            attributes: Vec::new(),
            supports: Vec::new(),
            editor_script: None,
            style: None,
            render: None,
        }
    }

    pub fn with_category(mut self, category: WordPressBlockCategory) -> Self {
        self.category = Some(category);
        self
    }

    pub fn with_attribute(mut self, attribute: WordPressBlockAttribute) -> Self {
        self.attributes.push(attribute);
        self
    }

    pub fn with_support(mut self, support: WordPressBlockSupport) -> Self {
        self.supports.push(support);
        self
    }

    pub fn with_render(mut self, render: WordPressBlockAssetPath) -> Self {
        self.render = Some(render);
        self
    }

    pub const fn name(&self) -> &WordPressBlockName {
        &self.name
    }

    pub const fn category(&self) -> Option<&WordPressBlockCategory> {
        self.category.as_ref()
    }

    pub fn attributes(&self) -> &[WordPressBlockAttribute] {
        &self.attributes
    }

    pub fn supports(&self) -> &[WordPressBlockSupport] {
        &self.supports
    }

    pub const fn render(&self) -> Option<&WordPressBlockAssetPath> {
        self.render.as_ref()
    }
}

/// Error returned when WordPress block metadata is invalid.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WordPressBlockError {
    Empty,
    InvalidBlockName,
}

impl fmt::Display for WordPressBlockError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => formatter.write_str("WordPress block metadata cannot be empty"),
            Self::InvalidBlockName => {
                formatter.write_str("WordPress block names must look like namespace/block")
            },
        }
    }
}

impl Error for WordPressBlockError {}

#[cfg(test)]
mod tests {
    use super::{
        WordPressBlockAttribute, WordPressBlockAttributeName, WordPressBlockAttributeType,
        WordPressBlockError, WordPressBlockJson, WordPressBlockName, WordPressBlockSupport,
    };

    #[test]
    fn builds_block_json_metadata() -> Result<(), WordPressBlockError> {
        let block = WordPressBlockJson::new(WordPressBlockName::new("acme/book-card")?)
            .with_attribute(WordPressBlockAttribute::new(
                WordPressBlockAttributeName::new("title")?,
                WordPressBlockAttributeType::String,
            ))
            .with_support(WordPressBlockSupport::Color);

        assert_eq!(block.name().as_str(), "acme/book-card");
        assert_eq!(block.attributes()[0].name().as_str(), "title");
        assert_eq!(block.supports(), &[WordPressBlockSupport::Color]);
        Ok(())
    }
}
