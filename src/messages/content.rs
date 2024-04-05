use std::fmt::Display;
use std::path::PathBuf;

use crate::macros::{
    impl_display_for_serialize, impl_enum_string_serialization,
    impl_enum_struct_serialization,
    impl_enum_with_string_or_array_serialization,
};
use crate::messages::{ContentFlatteningError, ImageMediaTypeParseError};

/// The content of the message.
///
/// ## Example
/// ```rust
/// use clust::messages::{Content, ContentBlock, ImageContentBlock, ImageContentSource, ImageMediaType, TextContentBlock, ToolUseContentBlock};
///
/// // Manual
/// let content = Content::SingleText("text".to_string());
/// let content = Content::MultipleBlocks(vec![ContentBlock::Text(TextContentBlock::new("text"))]);
/// let content = Content::MultipleBlocks(vec![
///     ContentBlock::Image(ImageContentBlock::new(ImageContentSource::base64(ImageMediaType::Png, "base64")))
/// ]);
/// let content = Content::MultipleBlocks(vec![
///     ContentBlock::ToolUse(ToolUseContentBlock::new("id", "name", serde_json::Value::Null)),
/// ]);
/// let content = Content::MultipleBlocks(vec![
///     ContentBlock::Text(TextContentBlock::new("text")),
///     ContentBlock::Image(ImageContentBlock::new(ImageContentSource::base64(ImageMediaType::Png, "base64"))),
/// ]);
///
/// // From trait
/// let content = Content::from("text");
/// let content = Content::from(vec![ContentBlock::from("text")]);
/// let content = Content::from(ImageContentSource::base64(ImageMediaType::Png, "base64"));
/// let content = Content::from(ToolUseContentBlock::new("id", "name", serde_json::Value::Null));
/// let content = Content::from(vec![
///     ContentBlock::from("text"),
///     ContentBlock::from(ImageContentSource::base64(ImageMediaType::Png, "base64")),
/// ]);
///
/// // Into trait
/// let content: Content = "text".into();
/// let content: Content = vec![ContentBlock::from("text")].into();
/// let content: Content = ImageContentSource::base64(ImageMediaType::Png, "base64").into();
/// let content: Content = ToolUseContentBlock::new("id", "name", serde_json::Value::Null).into();
/// let content: Content = vec![
///     "text".into(),
///     ImageContentSource::base64(ImageMediaType::Png, "base64").into(),
/// ].into();
/// ```
#[derive(Debug, Clone, PartialEq)]
pub enum Content {
    /// The single text content.
    SingleText(String),
    /// The multiple content blocks.
    MultipleBlocks(Vec<ContentBlock>),
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

impl From<ImageContentSource> for Content {
    fn from(image: ImageContentSource) -> Self {
        Self::MultipleBlocks(vec![ContentBlock::Image(
            image.into(),
        )])
    }
}

impl From<ToolUseContentBlock> for Content {
    fn from(tool_use: ToolUseContentBlock) -> Self {
        Self::MultipleBlocks(vec![ContentBlock::ToolUse(tool_use)])
    }
}

impl_enum_with_string_or_array_serialization!(
    Content,
    SingleText(String),
    MultipleBlocks(ContentBlock)
);

impl_display_for_serialize!(Content);

impl Content {
    /// Flattens the content into a single text.
    /// - `Content::SingleText` => Returns "`Ok(text)`"
    /// - `Content::MultipleBlock` =>
    ///     - Has `ContentBlock::Text` at the first block => Returns "`Ok(text)`"
    ///     - Otherwise => Returns "`Err(TextContentExtractionError)`".
    pub fn flatten_into_text(&self) -> Result<&str, ContentFlatteningError> {
        match self {
            | Content::SingleText(text) => Ok(text),
            | Content::MultipleBlocks(block) => match block.first() {
                | Some(first) => match first {
                    | ContentBlock::Text(text) => Ok(&text.text),
                    | _ => Err(ContentFlatteningError::NotFoundTargetBlock),
                },
                | None => Err(ContentFlatteningError::Empty),
            },
        }
    }

    /// Flattens the content into a single image source.
    /// - `Content::SingleText` => Returns "`Err(NotFoundTargetBlock)`"
    /// - `Content::MultipleBlock` =>
    ///     - Has `ContentBlock::Image` at the first block => Returns "`Ok(image_source)`"
    ///     - Otherwise => Returns "`Err(NotFoundTargetBlock)`".
    pub fn flatten_into_image_source(
        &self
    ) -> Result<&ImageContentSource, ContentFlatteningError> {
        match self {
            | Content::SingleText(_) => {
                Err(ContentFlatteningError::NotFoundTargetBlock)
            },
            | Content::MultipleBlocks(block) => match block.first() {
                | Some(first) => match first {
                    | ContentBlock::Image(image) => Ok(&image.source),
                    | _ => Err(ContentFlatteningError::NotFoundTargetBlock),
                },
                | None => Err(ContentFlatteningError::Empty),
            },
        }
    }

    /// Flattens the content into a single tool use.
    /// - `Content::SingleText` => Returns "`Err(NotFoundTargetBlock)`"
    /// - `Content::MultipleBlock` =>
    ///     - Has `ContentBlock::ToolUse` at the first block => Returns "`Ok(tool_use)`"
    ///     - Otherwise => Returns "`Err(NotFoundTargetBlock)`".
    pub fn flatten_into_tool_use(
        &self
    ) -> Result<ToolUseContentBlock, ContentFlatteningError> {
        match self {
            | Content::SingleText(_) => {
                Err(ContentFlatteningError::NotFoundTargetBlock)
            },
            | Content::MultipleBlocks(block) => match block.first() {
                | Some(first) => match first {
                    | ContentBlock::ToolUse(tool_use) => Ok(tool_use.clone()),
                    | _ => Err(ContentFlatteningError::NotFoundTargetBlock),
                },
                | None => Err(ContentFlatteningError::Empty),
            },
        }
    }
}

/// The content block of the message.
#[derive(Debug, Clone, PartialEq)]
pub enum ContentBlock {
    /// The text content block.
    Text(TextContentBlock),
    /// The image content block.
    Image(ImageContentBlock),
    /// The tool use content block.
    ToolUse(ToolUseContentBlock),
}

impl Default for ContentBlock {
    fn default() -> Self {
        Self::Text(TextContentBlock::default())
    }
}

impl From<String> for ContentBlock {
    fn from(text: String) -> Self {
        Self::Text(TextContentBlock::new(text))
    }
}

impl From<&str> for ContentBlock {
    fn from(text: &str) -> Self {
        Self::Text(TextContentBlock::new(text))
    }
}

impl From<ImageContentSource> for ContentBlock {
    fn from(image: ImageContentSource) -> Self {
        Self::Image(ImageContentBlock::new(image))
    }
}

impl_enum_struct_serialization!(
    ContentBlock,
    type,
    Text(TextContentBlock, "text"),
    Image(ImageContentBlock, "image"),
    ToolUse(ToolUseContentBlock, "tool_use")
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
        Self::new(text)
    }
}

impl From<&str> for TextContentBlock {
    fn from(text: &str) -> Self {
        Self::new(text)
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

impl From<ImageContentSource> for ImageContentBlock {
    fn from(source: ImageContentSource) -> Self {
        Self::new(source)
    }
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
    /// text_delta
    TextDelta,
    /// tool_use
    ToolUse,
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
            | ContentType::ToolUse => {
                write!(f, "tool_use")
            },
        }
    }
}

impl_enum_string_serialization!(
    ContentType,
    Text => "text",
    Image => "image",
    TextDelta => "text_delta",
    ToolUse => "tool_use"
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

impl ImageContentSource {
    /// Creates a new image content source from Base64 encoded image data.
    ///
    /// ## Arguments
    /// - `media_type` - The media type of the image.
    /// - `data` - The data of the image.
    pub fn base64<S>(
        media_type: ImageMediaType,
        data: S,
    ) -> Self
    where
        S: Into<String>,
    {
        Self {
            _type: ImageSourceType::Base64,
            media_type,
            data: data.into(),
        }
    }
}

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

impl ImageMediaType {
    /// Creates the media type from the extension of the path.
    pub fn from_path(path: &PathBuf) -> Result<Self, ImageMediaTypeParseError> {
        match path
            .extension()
            .and_then(|ext| ext.to_str())
        {
            | Some("jpeg") | Some("jpg") => Ok(Self::Jpeg),
            | Some("png") => Ok(Self::Png),
            | Some("gif") => Ok(Self::Gif),
            | Some("webp") => Ok(Self::Webp),
            | Some(extension) => Err(ImageMediaTypeParseError::NotSupported(
                extension.to_string(),
            )),
            | None => Err(ImageMediaTypeParseError::NotFound),
        }
    }
}

/// The tool use content block.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ToolUseContentBlock {
    /// The content type. It is always `tool_use`.
    #[serde(rename = "type")]
    pub _type: ContentType,
    /// The ID of the used tool.
    pub id: String,
    /// The name of the used tool.
    pub name: String,
    /// The input of the used tool.
    pub input: serde_json::Value,
}

impl Default for ToolUseContentBlock {
    fn default() -> Self {
        Self {
            _type: ContentType::ToolUse,
            id: String::new(),
            name: String::new(),
            input: serde_json::Value::Null,
        }
    }
}

impl_display_for_serialize!(ToolUseContentBlock);

impl ToolUseContentBlock {
    /// Creates a new tool use content block.
    pub fn new<S, T>(
        id: S,
        name: T,
        input: serde_json::Value,
    ) -> Self
    where
        S: Into<String>,
        T: Into<String>,
    {
        Self {
            _type: ContentType::ToolUse,
            id: id.into(),
            name: name.into(),
            input,
        }
    }
}

#[cfg(test)]
mod tests {
    use serde_json::Map;

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
        assert_eq!(
            ContentType::ToolUse.to_string(),
            "tool_use"
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
        assert_eq!(
            serde_json::to_string(&ContentType::ToolUse).unwrap(),
            "\"tool_use\""
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
        assert_eq!(
            serde_json::from_str::<ContentType>("\"tool_use\"").unwrap(),
            ContentType::ToolUse
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
    fn from_path_image_media_type() {
        let path = PathBuf::from("image.jpeg");
        assert_eq!(
            ImageMediaType::from_path(&path).unwrap(),
            ImageMediaType::Jpeg
        );

        let path = PathBuf::from("image.png");
        assert_eq!(
            ImageMediaType::from_path(&path).unwrap(),
            ImageMediaType::Png
        );

        let path = PathBuf::from("image.gif");
        assert_eq!(
            ImageMediaType::from_path(&path).unwrap(),
            ImageMediaType::Gif
        );

        let path = PathBuf::from("image.webp");
        assert_eq!(
            ImageMediaType::from_path(&path).unwrap(),
            ImageMediaType::Webp
        );

        let path = PathBuf::from("image.bmp");
        assert_eq!(
            ImageMediaType::from_path(&path),
            Err(ImageMediaTypeParseError::NotSupported(
                "bmp".to_string()
            ))
        );

        let path = PathBuf::from("image");
        assert!(ImageMediaType::from_path(&path).is_err());
    }

    #[test]
    fn new_image_content_source() {
        let image_content_source =
            ImageContentSource::base64(ImageMediaType::Jpeg, "data");
        assert_eq!(
            image_content_source,
            ImageContentSource {
                _type: ImageSourceType::Base64,
                media_type: ImageMediaType::Jpeg,
                data: "data".to_string(),
            }
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
    fn new_tool_use_content_block() {
        let tool_use_content_block = ToolUseContentBlock::new(
            "id",
            "name",
            serde_json::Value::Object(Map::new()),
        );
        assert_eq!(
            tool_use_content_block,
            ToolUseContentBlock {
                _type: ContentType::ToolUse,
                id: "id".to_string(),
                name: "name".to_string(),
                input: serde_json::Value::Object(Map::new()),
            }
        );
    }

    #[test]
    fn default_tool_use_content_block() {
        assert_eq!(
            ToolUseContentBlock::default(),
            ToolUseContentBlock {
                _type: ContentType::ToolUse,
                id: String::new(),
                name: String::new(),
                input: serde_json::Value::Null,
            }
        );
    }

    #[test]
    fn display_tool_use_content_block() {
        let tool_use_content_block = ToolUseContentBlock {
            _type: ContentType::ToolUse,
            id: "id".to_string(),
            name: "name".to_string(),
            input: serde_json::Value::Object(Map::new()),
        };
        assert_eq!(
            tool_use_content_block.to_string(),
            "{\n  \"type\": \"tool_use\",\n  \"id\": \"id\",\n  \"name\": \"name\",\n  \"input\": {}\n}"
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

        let content_block = ContentBlock::ToolUse(ToolUseContentBlock::new(
            "id",
            "name",
            serde_json::Value::Object(Map::new()),
        ));
        assert_eq!(
            content_block,
            ContentBlock::ToolUse(ToolUseContentBlock {
                _type: ContentType::ToolUse,
                id: "id".to_string(),
                name: "name".to_string(),
                input: serde_json::Value::Object(Map::new()),
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

        let content_block = ContentBlock::ToolUse(ToolUseContentBlock::new(
            "id",
            "name",
            serde_json::Value::Object(Map::new()),
        ));
        assert_eq!(
            content_block.to_string(),
            "{\n  \"type\": \"tool_use\",\n  \"id\": \"id\",\n  \"name\": \"name\",\n  \"input\": {}\n}"
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

        let content_block = ContentBlock::ToolUse(ToolUseContentBlock::new(
            "id",
            "name",
            serde_json::Value::Object(Map::new()),
        ));
        assert_eq!(
            serde_json::to_string(&content_block).unwrap(),
            "{\"type\":\"tool_use\",\"id\":\"id\",\"name\":\"name\",\"input\":{}}"
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

        let content_block = ContentBlock::ToolUse(ToolUseContentBlock::new(
            "id",
            "name",
            serde_json::Value::Object(Map::new()),
        ));
        assert_eq!(
            serde_json::from_str::<ContentBlock>("{\"type\":\"tool_use\",\"id\":\"id\",\"name\":\"name\",\"input\":{}}").unwrap(),
            content_block
        );
    }

    #[test]
    fn from_content_block() {
        assert_eq!(
            ContentBlock::from("text"),
            ContentBlock::Text(TextContentBlock::new("text"))
        );

        let content_block: ContentBlock = "text".into();
        assert_eq!(
            content_block,
            ContentBlock::Text(TextContentBlock::new("text"))
        );

        assert_eq!(
            ContentBlock::from(ImageContentSource::default()),
            ContentBlock::Image(ImageContentBlock::new(
                ImageContentSource::default()
            ))
        );

        let content_block: ContentBlock = ImageContentSource::default().into();
        assert_eq!(
            content_block,
            ContentBlock::Image(ImageContentBlock::new(
                ImageContentSource::default()
            ))
        );
    }

    #[test]
    fn new_content() {
        let content = Content::SingleText("text".to_string());
        assert_eq!(
            content,
            Content::SingleText("text".to_string())
        );

        let content = Content::MultipleBlocks(vec![
            ContentBlock::Text(TextContentBlock::new(
                "text".to_string(),
            )),
            ContentBlock::Image(ImageContentBlock::new(
                ImageContentSource::default(),
            )),
        ]);
        assert_eq!(
            content,
            Content::MultipleBlocks(vec![
                ContentBlock::Text(TextContentBlock::new(
                    "text".to_string(),
                )),
                ContentBlock::Image(ImageContentBlock::new(
                    ImageContentSource::default(),
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

        let content = Content::MultipleBlocks(vec![
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

        let content = Content::MultipleBlocks(vec![
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

        let content = Content::MultipleBlocks(vec![
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

    #[test]
    fn from_content() {
        assert_eq!(
            Content::from("text"),
            Content::SingleText("text".to_string())
        );

        let content: Content = "text".into();
        assert_eq!(
            content,
            Content::SingleText("text".to_string())
        );

        assert_eq!(
            Content::from(vec![ContentBlock::from(
                "text"
            )]),
            Content::MultipleBlocks(vec![ContentBlock::Text(
                TextContentBlock::new("text")
            )])
        );

        let content: Content = vec!["text".into()].into();
        assert_eq!(
            content,
            Content::MultipleBlocks(vec![ContentBlock::Text(
                TextContentBlock::new("text")
            )])
        );

        assert_eq!(
            Content::from(ImageContentSource::default()),
            Content::MultipleBlocks(vec![ContentBlock::Image(
                ImageContentBlock::new(ImageContentSource::default())
            )])
        );

        let content: Content = ImageContentSource::default().into();
        assert_eq!(
            content,
            Content::MultipleBlocks(vec![ContentBlock::Image(
                ImageContentBlock::new(ImageContentSource::default())
            )])
        );
    }

    #[test]
    fn flatten_into_text() {
        assert_eq!(
            Content::from("text")
                .flatten_into_text()
                .unwrap(),
            "text"
        );

        assert_eq!(
            Content::from(vec![
                ContentBlock::from("text"),
                ContentBlock::from(ImageContentSource::default()),
            ])
            .flatten_into_text()
            .unwrap(),
            "text"
        );

        assert_eq!(
            Content::from(vec![
                ContentBlock::from("text"),
                ContentBlock::from("second"),
            ])
            .flatten_into_text()
            .unwrap(),
            "text"
        );

        assert!(Content::from(vec![])
            .flatten_into_text()
            .is_err());

        assert!(
            Content::from(ImageContentSource::default())
                .flatten_into_text()
                .is_err()
        );

        assert!(Content::from(vec![
            ContentBlock::from(ImageContentSource::default()),
            ContentBlock::from("text"),
        ])
        .flatten_into_text()
        .is_err());
    }

    #[test]
    fn flatten_into_image_source() {
        assert!(Content::from("text")
            .flatten_into_image_source()
            .is_err());

        assert!(Content::from(vec![
            ContentBlock::from("text"),
            ContentBlock::from(ImageContentSource::default()),
        ])
        .flatten_into_image_source()
        .is_err());

        assert!(Content::from(vec![
            ContentBlock::from("text"),
            ContentBlock::from("second"),
        ])
        .flatten_into_image_source()
        .is_err());

        assert!(Content::from(vec![])
            .flatten_into_image_source()
            .is_err());

        assert_eq!(
            *Content::from(ImageContentSource::default())
                .flatten_into_image_source()
                .unwrap(),
            ImageContentSource::default()
        );

        assert_eq!(
            *Content::from(vec![
                ContentBlock::from(ImageContentSource::default()),
                ContentBlock::from("text"),
            ])
            .flatten_into_image_source()
            .unwrap(),
            ImageContentSource::default()
        );
    }

    #[test]
    fn flatten_into_tool_use() {
        assert!(Content::from("text")
            .flatten_into_tool_use()
            .is_err());

        assert!(Content::from(vec![
            ContentBlock::from("text"),
            ContentBlock::from(ImageContentSource::default()),
        ])
        .flatten_into_tool_use()
        .is_err());

        assert!(Content::from(vec![
            ContentBlock::from("text"),
            ContentBlock::from("second"),
        ])
        .flatten_into_tool_use()
        .is_err());

        assert!(Content::from(vec![])
            .flatten_into_tool_use()
            .is_err());

        assert!(
            Content::from(ImageContentSource::default())
                .flatten_into_tool_use()
                .is_err()
        );

        assert_eq!(
            Content::from(vec![
                ContentBlock::from(ToolUseContentBlock::new(
                    "id",
                    "name",
                    serde_json::Value::Object(Map::new())
                )),
                ContentBlock::from("text"),
            ])
            .flatten_into_tool_use()
            .unwrap(),
            ToolUseContentBlock::new(
                "id",
                "name",
                serde_json::Value::Object(Map::new())
            )
        );
    }
}
