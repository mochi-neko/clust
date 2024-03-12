use crate::macros::{
    impl_display_for_serialize, impl_enum_string_serialization,
    impl_enum_struct_serialization,
    impl_enum_with_string_or_array_serialization,
};
use std::fmt::Display;

/// The content of the message.
#[derive(Debug, Clone, PartialEq)]
pub enum Content {
    /// The single text content.
    SingleText(String),
    /// The multiple content blocks.
    MultipleBlock(Vec<ContentBlock>),
}

impl From<ContentBlock> for Content {
    fn from(block: ContentBlock) -> Self {
        Self::MultipleBlock(vec![block])
    }
}

impl From<&str> for Content {
    fn from(text: &str) -> Self {
        Self::SingleText(text.to_string())
    }
}

impl_enum_with_string_or_array_serialization!(
    Content,
    SingleText(String),
    MultipleBlock(ContentBlock)
);

/// The content block of the message.
#[derive(Debug, Clone, PartialEq)]
pub enum ContentBlock {
    /// The text content block.
    Text(TextContentBlock),
    /// The image content block.
    Image(ImageContentBlock),
}

impl_enum_struct_serialization!(
    ContentBlock,
    type,
    Text(TextContentBlock, "text"),
    Image(ImageContentBlock, "image")
);

/// The text content block.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct TextContentBlock {
    /// The content type. It is always `text`.
    #[serde(rename = "type")]
    pub _type: ContentType,
    /// The text content.
    pub text: String,
}

impl TextContentBlock {
    /// Creates a new text content block.
    pub fn new(text: String) -> Self {
        Self {
            _type: ContentType::Text,
            text,
        }
    }
}

/// The image content block.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ImageContentBlock {
    /// The content type. It is always `image`.
    #[serde(rename = "type")]
    pub _type: ContentType,
    /// The image content source.
    pub source: ImageContentSource,
}

impl ImageContentBlock {
    /// Creates a new image content block.
    pub fn new(source: ImageContentSource) -> Self {
        Self {
            _type: ContentType::Image,
            source,
        }
    }
}

/// The content type of the message.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ContentType {
    /// text
    Text,
    /// image
    Image,
}

impl Default for ContentType {
    fn default() -> Self {
        Self::Text
    }
}

impl Display for ContentType {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        match self {
            | ContentType::Text => {
                write!(f, "text")
            },
            | ContentType::Image => {
                write!(f, "image")
            },
        }
    }
}

impl_enum_string_serialization!(
    ContentType,
    Text => "text",
    Image => "image"
);

/// The image content source.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ImageContentSource {
    /// The source type.
    #[serde(rename = "type")]
    pub _type: ImageSourceType,
    /// The media type.
    pub media_type: ImageMediaType,
    ///  The data of the image.
    pub data: String,
}

impl_display_for_serialize!(ImageContentSource);

/// The source type of the image.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ImageSourceType {
    /// base64
    Base64,
}

impl Default for ImageSourceType {
    fn default() -> Self {
        Self::Base64
    }
}

impl Display for ImageSourceType {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        match self {
            | ImageSourceType::Base64 => {
                write!(f, "base64")
            },
        }
    }
}

impl_enum_string_serialization!(
    ImageSourceType,
    Base64 => "base64"
);

/// The media type of the image.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ImageMediaType {
    /// image/jpeg
    Jpeg,
    /// image/png
    Png,
    /// image/gif
    Gif,
    /// image/webp
    Webp,
}

impl Default for ImageMediaType {
    fn default() -> Self {
        Self::Jpeg
    }
}

impl Display for ImageMediaType {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        match self {
            | ImageMediaType::Jpeg => {
                write!(f, "image/jpeg")
            },
            | ImageMediaType::Png => {
                write!(f, "image/png")
            },
            | ImageMediaType::Gif => {
                write!(f, "image/gif")
            },
            | ImageMediaType::Webp => {
                write!(f, "image/webp")
            },
        }
    }
}

impl_enum_string_serialization!(
    ImageMediaType,
    Jpeg => "image/jpeg",
    Png => "image/png",
    Gif => "image/gif",
    Webp => "image/webp"
);
