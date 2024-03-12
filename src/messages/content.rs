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

impl Default for Content {
    fn default() -> Self {
        Self::SingleText(String::new())
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

impl_display_for_serialize!(Content);

/// The content block of the message.
#[derive(Debug, Clone, PartialEq)]
pub enum ContentBlock {
    /// The text content block.
    Text(TextContentBlock),
    /// The image content block.
    Image(ImageContentBlock),
    /// The text delta content block.
    TextDelta(TextDeltaContentBlock),
}

impl Default for ContentBlock {
    fn default() -> Self {
        Self::Text(TextContentBlock::default())
    }
}

impl_enum_struct_serialization!(
    ContentBlock,
    type,
    Text(TextContentBlock, "text"),
    Image(ImageContentBlock, "image"),
    TextDelta(TextDeltaContentBlock, "text_delta")
);

impl_display_for_serialize!(ContentBlock);

/// The text content block.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct TextContentBlock {
    /// The content type. It is always `text`.
    #[serde(rename = "type")]
    pub _type: ContentType,
    /// The text content.
    pub text: String,
}

impl Default for TextContentBlock {
    fn default() -> Self {
        Self {
            _type: ContentType::Text,
            text: String::new(),
        }
    }
}

impl_display_for_serialize!(TextContentBlock);

impl From<String> for TextContentBlock {
    fn from(text: String) -> Self {
        Self {
            _type: ContentType::Text,
            text,
        }
    }
}

impl From<&str> for TextContentBlock {
    fn from(text: &str) -> Self {
        Self {
            _type: ContentType::Text,
            text: text.to_string(),
        }
    }
}

impl TextContentBlock {
    /// Creates a new text content block.
    pub fn new<S>(text: S) -> Self
    where
        S: Into<String>,
    {
        Self {
            _type: ContentType::Text,
            text: text.into(),
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

impl Default for ImageContentBlock {
    fn default() -> Self {
        Self {
            _type: ContentType::Image,
            source: ImageContentSource::default(),
        }
    }
}

impl_display_for_serialize!(ImageContentBlock);

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
    /// text_delta
    TextDelta,
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
            | ContentType::TextDelta => {
                write!(f, "text_delta")
            },
        }
    }
}

impl_enum_string_serialization!(
    ContentType,
    Text => "text",
    Image => "image",
    TextDelta => "text_delta"
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

impl Default for ImageContentSource {
    fn default() -> Self {
        Self {
            _type: ImageSourceType::default(),
            media_type: ImageMediaType::default(),
            data: String::new(),
        }
    }
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

/// The text delta content block.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct TextDeltaContentBlock {
    /// The content type. It is always `text_delta`.
    #[serde(rename = "type")]
    pub _type: ContentType,
    /// The text delta content.
    pub text: String,
}

impl Default for TextDeltaContentBlock {
    fn default() -> Self {
        Self {
            _type: ContentType::TextDelta,
            text: String::new(),
        }
    }
}

impl_display_for_serialize!(TextDeltaContentBlock);

impl From<String> for TextDeltaContentBlock {
    fn from(text: String) -> Self {
        Self::new(text)
    }
}

impl From<&str> for TextDeltaContentBlock {
    fn from(text: &str) -> Self {
        Self::new(text)
    }
}

impl TextDeltaContentBlock {
    /// Creates a new text content block.
    pub(crate) fn new<S>(text: S) -> Self
    where
        S: Into<String>,
    {
        Self {
            _type: ContentType::TextDelta,
            text: text.into(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_str() {
        assert_eq!(
            Content::from("text"),
            Content::SingleText("text".to_string())
        );
    }

    #[test]
    fn default_content_type() {
        assert_eq!(
            ContentType::default(),
            ContentType::Text
        );
    }

    #[test]
    fn display_content_type() {
        assert_eq!(ContentType::Text.to_string(), "text");
        assert_eq!(ContentType::Image.to_string(), "image");
        assert_eq!(
            ContentType::TextDelta.to_string(),
            "text_delta"
        );
    }

    #[test]
    fn serialize_content_type() {
        assert_eq!(
            serde_json::to_string(&ContentType::Text).unwrap(),
            "\"text\""
        );
        assert_eq!(
            serde_json::to_string(&ContentType::Image).unwrap(),
            "\"image\""
        );
        assert_eq!(
            serde_json::to_string(&ContentType::TextDelta).unwrap(),
            "\"text_delta\""
        );
    }

    #[test]
    fn deserialize_content_type() {
        assert_eq!(
            serde_json::from_str::<ContentType>("\"text\"").unwrap(),
            ContentType::Text
        );
        assert_eq!(
            serde_json::from_str::<ContentType>("\"image\"").unwrap(),
            ContentType::Image
        );
        assert_eq!(
            serde_json::from_str::<ContentType>("\"text_delta\"").unwrap(),
            ContentType::TextDelta
        );
    }

    #[test]
    fn default_image_source_type() {
        assert_eq!(
            ImageSourceType::default(),
            ImageSourceType::Base64
        );
    }

    #[test]
    fn display_image_source_type() {
        assert_eq!(
            ImageSourceType::Base64.to_string(),
            "base64"
        );
    }

    #[test]
    fn serialize_image_source_type() {
        assert_eq!(
            serde_json::to_string(&ImageSourceType::Base64).unwrap(),
            "\"base64\""
        );
    }

    #[test]
    fn deserialize_image_source_type() {
        assert_eq!(
            serde_json::from_str::<ImageSourceType>("\"base64\"").unwrap(),
            ImageSourceType::Base64
        );
    }

    #[test]
    fn default_image_media_type() {
        assert_eq!(
            ImageMediaType::default(),
            ImageMediaType::Jpeg
        );
    }

    #[test]
    fn display_image_media_type() {
        assert_eq!(
            ImageMediaType::Jpeg.to_string(),
            "image/jpeg"
        );
        assert_eq!(
            ImageMediaType::Png.to_string(),
            "image/png"
        );
        assert_eq!(
            ImageMediaType::Gif.to_string(),
            "image/gif"
        );
        assert_eq!(
            ImageMediaType::Webp.to_string(),
            "image/webp"
        );
    }

    #[test]
    fn serialize_image_media_type() {
        assert_eq!(
            serde_json::to_string(&ImageMediaType::Jpeg).unwrap(),
            "\"image/jpeg\""
        );
        assert_eq!(
            serde_json::to_string(&ImageMediaType::Png).unwrap(),
            "\"image/png\""
        );
        assert_eq!(
            serde_json::to_string(&ImageMediaType::Gif).unwrap(),
            "\"image/gif\""
        );
        assert_eq!(
            serde_json::to_string(&ImageMediaType::Webp).unwrap(),
            "\"image/webp\""
        );
    }

    #[test]
    fn deserialize_image_media_type() {
        assert_eq!(
            serde_json::from_str::<ImageMediaType>("\"image/jpeg\"").unwrap(),
            ImageMediaType::Jpeg
        );
        assert_eq!(
            serde_json::from_str::<ImageMediaType>("\"image/png\"").unwrap(),
            ImageMediaType::Png
        );
        assert_eq!(
            serde_json::from_str::<ImageMediaType>("\"image/gif\"").unwrap(),
            ImageMediaType::Gif
        );
        assert_eq!(
            serde_json::from_str::<ImageMediaType>("\"image/webp\"").unwrap(),
            ImageMediaType::Webp
        );
    }

    #[test]
    fn default_image_content_source() {
        assert_eq!(
            ImageContentSource::default(),
            ImageContentSource {
                _type: ImageSourceType::Base64,
                media_type: ImageMediaType::Jpeg,
                data: String::new(),
            }
        );
    }

    #[test]
    fn display_image_content_source() {
        let image_content_source = ImageContentSource {
            _type: ImageSourceType::Base64,
            media_type: ImageMediaType::Jpeg,
            data: "data".to_string(),
        };
        assert_eq!(
            image_content_source.to_string(),
            "{\n  \"type\": \"base64\",\n  \"media_type\": \"image/jpeg\",\n  \"data\": \"data\"\n}"
        );
    }

    #[test]
    fn serialize_image_content_source() {
        let image_content_source = ImageContentSource {
            _type: ImageSourceType::Base64,
            media_type: ImageMediaType::Jpeg,
            data: "data".to_string(),
        };
        assert_eq!(
            serde_json::to_string(&image_content_source).unwrap(),
            "{\"type\":\"base64\",\"media_type\":\"image/jpeg\",\"data\":\"data\"}"
        );
    }

    #[test]
    fn deserialize_image_content_source() {
        let image_content_source = ImageContentSource {
            _type: ImageSourceType::Base64,
            media_type: ImageMediaType::Jpeg,
            data: "data".to_string(),
        };
        assert_eq!(
            serde_json::from_str::<ImageContentSource>("{\"type\":\"base64\",\"media_type\":\"image/jpeg\",\"data\":\"data\"}").unwrap(),
            image_content_source
        );
    }

    #[test]
    fn new_text_content_block() {
        let text_content_block = TextContentBlock::new("text".to_string());
        assert_eq!(
            text_content_block,
            TextContentBlock {
                _type: ContentType::Text,
                text: "text".to_string(),
            }
        );
    }

    #[test]
    fn default_text_content_block() {
        assert_eq!(
            TextContentBlock::default(),
            TextContentBlock {
                _type: ContentType::Text,
                text: String::new(),
            }
        );
    }

    #[test]
    fn display_text_content_block() {
        let text_content_block = TextContentBlock::new("text".to_string());
        assert_eq!(
            text_content_block.to_string(),
            "{\n  \"type\": \"text\",\n  \"text\": \"text\"\n}"
        );
    }

    #[test]
    fn serialize_text_content_block() {
        let text_content_block = TextContentBlock::new("text".to_string());
        assert_eq!(
            serde_json::to_string(&text_content_block).unwrap(),
            "{\"type\":\"text\",\"text\":\"text\"}"
        );
    }

    #[test]
    fn deserialize_text_content_block() {
        let text_content_block = TextContentBlock::new("text".to_string());
        assert_eq!(
            serde_json::from_str::<TextContentBlock>(
                "{\"type\":\"text\",\"text\":\"text\"}"
            )
            .unwrap(),
            text_content_block
        );
    }

    #[test]
    fn new_image_content_block() {
        let image_content_block =
            ImageContentBlock::new(ImageContentSource::default());
        assert_eq!(
            image_content_block,
            ImageContentBlock {
                _type: ContentType::Image,
                source: ImageContentSource::default(),
            }
        );
    }

    #[test]
    fn default_image_content_block() {
        assert_eq!(
            ImageContentBlock::default(),
            ImageContentBlock {
                _type: ContentType::Image,
                source: ImageContentSource::default(),
            }
        );
    }

    #[test]
    fn display_image_content_block() {
        let image_content_block =
            ImageContentBlock::new(ImageContentSource::default());
        assert_eq!(
            image_content_block.to_string(),
            "{\n  \"type\": \"image\",\n  \"source\": {\n    \"type\": \"base64\",\n    \"media_type\": \"image/jpeg\",\n    \"data\": \"\"\n  }\n}"
        );
    }

    #[test]
    fn serialize_image_content_block() {
        let image_content_block =
            ImageContentBlock::new(ImageContentSource::default());
        assert_eq!(
            serde_json::to_string(&image_content_block).unwrap(),
            "{\"type\":\"image\",\"source\":{\"type\":\"base64\",\"media_type\":\"image/jpeg\",\"data\":\"\"}}"
        );
    }

    #[test]
    fn deserialize_image_content_block() {
        let image_content_block =
            ImageContentBlock::new(ImageContentSource::default());
        assert_eq!(
            serde_json::from_str::<ImageContentBlock>(
                "{\"type\":\"image\",\"source\":{\"type\":\"base64\",\"media_type\":\"image/jpeg\",\"data\":\"\"}}"
            )
            .unwrap(),
            image_content_block
        );
    }

    #[test]
    fn new_text_delta_content_block() {
        let text_delta_content_block =
            TextDeltaContentBlock::new("text".to_string());
        assert_eq!(
            text_delta_content_block,
            TextDeltaContentBlock {
                _type: ContentType::TextDelta,
                text: "text".to_string(),
            }
        );
    }

    #[test]
    fn default_text_delta_content_block() {
        assert_eq!(
            TextDeltaContentBlock::default(),
            TextDeltaContentBlock {
                _type: ContentType::TextDelta,
                text: String::new(),
            }
        );
    }

    #[test]
    fn display_text_delta_content_block() {
        let text_delta_content_block =
            TextDeltaContentBlock::new("text".to_string());
        assert_eq!(
            text_delta_content_block.to_string(),
            "{\n  \"type\": \"text_delta\",\n  \"text\": \"text\"\n}"
        );
    }

    #[test]
    fn serialize_text_delta_content_block() {
        let text_delta_content_block =
            TextDeltaContentBlock::new("text".to_string());
        assert_eq!(
            serde_json::to_string(&text_delta_content_block).unwrap(),
            "{\"type\":\"text_delta\",\"text\":\"text\"}"
        );
    }

    #[test]
    fn deserialize_text_delta_content_block() {
        let text_delta_content_block =
            TextDeltaContentBlock::new("text".to_string());
        assert_eq!(
            serde_json::from_str::<TextDeltaContentBlock>(
                "{\"type\":\"text_delta\",\"text\":\"text\"}"
            )
            .unwrap(),
            text_delta_content_block
        );
    }

    #[test]
    fn new_content_block() {
        let content_block = ContentBlock::Text(TextContentBlock::new(
            "text".to_string(),
        ));
        assert_eq!(
            content_block,
            ContentBlock::Text(TextContentBlock {
                _type: ContentType::Text,
                text: "text".to_string(),
            })
        );

        let content_block = ContentBlock::Image(ImageContentBlock::new(
            ImageContentSource::default(),
        ));
        assert_eq!(
            content_block,
            ContentBlock::Image(ImageContentBlock {
                _type: ContentType::Image,
                source: ImageContentSource::default(),
            })
        );

        let content_block = ContentBlock::TextDelta(
            TextDeltaContentBlock::new("text".to_string()),
        );
        assert_eq!(
            content_block,
            ContentBlock::TextDelta(TextDeltaContentBlock {
                _type: ContentType::TextDelta,
                text: "text".to_string(),
            })
        );
    }

    #[test]
    fn default_content_block() {
        assert_eq!(
            ContentBlock::default(),
            ContentBlock::Text(TextContentBlock::default())
        );
    }

    #[test]
    fn display_content_block() {
        let content_block = ContentBlock::Text(TextContentBlock::new(
            "text".to_string(),
        ));
        assert_eq!(
            content_block.to_string(),
            "{\n  \"type\": \"text\",\n  \"text\": \"text\"\n}"
        );

        let content_block = ContentBlock::Image(ImageContentBlock::new(
            ImageContentSource::default(),
        ));
        assert_eq!(
            content_block.to_string(),
            "{\n  \"type\": \"image\",\n  \"source\": {\n    \"type\": \"base64\",\n    \"media_type\": \"image/jpeg\",\n    \"data\": \"\"\n  }\n}"
        );

        let content_block = ContentBlock::TextDelta(
            TextDeltaContentBlock::new("text".to_string()),
        );
        assert_eq!(
            content_block.to_string(),
            "{\n  \"type\": \"text_delta\",\n  \"text\": \"text\"\n}"
        );
    }

    #[test]
    fn serialize_content_block() {
        let content_block = ContentBlock::Text(TextContentBlock::new(
            "text".to_string(),
        ));
        assert_eq!(
            serde_json::to_string(&content_block).unwrap(),
            "{\"type\":\"text\",\"text\":\"text\"}"
        );

        let content_block = ContentBlock::Image(ImageContentBlock::new(
            ImageContentSource::default(),
        ));
        assert_eq!(
            serde_json::to_string(&content_block).unwrap(),
            "{\"type\":\"image\",\"source\":{\"type\":\"base64\",\"media_type\":\"image/jpeg\",\"data\":\"\"}}"
        );

        let content_block = ContentBlock::TextDelta(
            TextDeltaContentBlock::new("text".to_string()),
        );
        assert_eq!(
            serde_json::to_string(&content_block).unwrap(),
            "{\"type\":\"text_delta\",\"text\":\"text\"}"
        );
    }

    #[test]
    fn deserialize_content_block() {
        let content_block = ContentBlock::Text(TextContentBlock::new(
            "text".to_string(),
        ));
        assert_eq!(
            serde_json::from_str::<ContentBlock>(
                "{\"type\":\"text\",\"text\":\"text\"}"
            )
            .unwrap(),
            content_block
        );

        let content_block = ContentBlock::Image(ImageContentBlock::new(
            ImageContentSource::default(),
        ));
        assert_eq!(
            serde_json::from_str::<ContentBlock>("{\"type\":\"image\",\"source\":{\"type\":\"base64\",\"media_type\":\"image/jpeg\",\"data\":\"\"}}").unwrap(),
            content_block
        );

        let content_block = ContentBlock::TextDelta(
            TextDeltaContentBlock::new("text".to_string()),
        );
        assert_eq!(
            serde_json::from_str::<ContentBlock>(
                "{\"type\":\"text_delta\",\"text\":\"text\"}"
            )
            .unwrap(),
            content_block
        );
    }

    #[test]
    fn new_content() {
        let content = Content::SingleText("text".to_string());
        assert_eq!(
            content,
            Content::SingleText("text".to_string())
        );

        let content = Content::MultipleBlock(vec![
            ContentBlock::Text(TextContentBlock::new(
                "text".to_string(),
            )),
            ContentBlock::Image(ImageContentBlock::new(
                ImageContentSource::default(),
            )),
        ]);
        assert_eq!(
            content,
            Content::MultipleBlock(vec![
                ContentBlock::Text(TextContentBlock::new(
                    "text".to_string()
                )),
                ContentBlock::Image(ImageContentBlock::new(
                    ImageContentSource::default()
                )),
            ])
        );
    }

    #[test]
    fn default_content() {
        assert_eq!(
            Content::default(),
            Content::SingleText(String::new())
        );
    }

    #[test]
    fn display_content() {
        let content = Content::SingleText("text".to_string());
        assert_eq!(content.to_string(), "\"text\"");

        let content = Content::MultipleBlock(vec![
            ContentBlock::Text(TextContentBlock::new(
                "text".to_string(),
            )),
            ContentBlock::Image(ImageContentBlock::new(
                ImageContentSource::default(),
            )),
        ]);
        assert_eq!(
            content.to_string(),
            "[\n  {\n    \"type\": \"text\",\n    \"text\": \"text\"\n  },\n  {\n    \"type\": \"image\",\n    \"source\": {\n      \"type\": \"base64\",\n      \"media_type\": \"image/jpeg\",\n      \"data\": \"\"\n    }\n  }\n]"
        );
    }

    #[test]
    fn serialize_content() {
        let content = Content::SingleText("text".to_string());
        assert_eq!(
            serde_json::to_string(&content).unwrap(),
            "\"text\""
        );

        let content = Content::MultipleBlock(vec![
            ContentBlock::Text(TextContentBlock::new(
                "text".to_string(),
            )),
            ContentBlock::Image(ImageContentBlock::new(
                ImageContentSource::default(),
            )),
        ]);
        assert_eq!(
            serde_json::to_string(&content).unwrap(),
            "[{\"type\":\"text\",\"text\":\"text\"},{\"type\":\"image\",\"source\":{\"type\":\"base64\",\"media_type\":\"image/jpeg\",\"data\":\"\"}}]"
        );
    }

    #[test]
    fn deserialize_content() {
        let content = Content::SingleText("text".to_string());
        assert_eq!(
            serde_json::from_str::<Content>("\"text\"").unwrap(),
            content
        );

        let content = Content::MultipleBlock(vec![
            ContentBlock::Text(TextContentBlock::new(
                "text".to_string(),
            )),
            ContentBlock::Image(ImageContentBlock::new(
                ImageContentSource::default(),
            )),
        ]);
        assert_eq!(
            serde_json::from_str::<Content>("[{\"type\":\"text\",\"text\":\"text\"},{\"type\":\"image\",\"source\":{\"type\":\"base64\",\"media_type\":\"image/jpeg\",\"data\":\"\"}}]").unwrap(),
            content
        );
    }
}
