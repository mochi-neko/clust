use crate::{ApiError, ClientError};

/// The error type for the messages API.
#[derive(Debug, thiserror::Error)]
pub enum MessagesError {
    /// The client error.
    #[error(transparent)]
    ClientError(#[from] ClientError),
    /// The API error.
    #[error(transparent)]
    ApiError(#[from] ApiError),
    /// Stream option mismatch.
    #[error("stream option mismatch")]
    StreamOptionMismatch,
}

/// The error type for the streaming messages.
#[derive(Debug, thiserror::Error)]
pub enum StreamError {
    /// Reqwest error.
    #[error(transparent)]
    ReqwestError(#[from] reqwest::Error),
    /// String decoding error.
    #[error(transparent)]
    StringDecodingError(#[from] std::string::FromUtf8Error),
    /// Parse chunk string error.
    #[error("parse chunk string error: {0}")]
    ParseChunkStringError(String),
    /// Chunk data deserialization error.
    #[error(transparent)]
    ChunkDataDeserializationError(#[from] serde_json::Error),
}

/// The error type for the text content extraction from response body of the Messages API.
#[derive(Debug, PartialEq, thiserror::Error)]
pub enum TextContentExtractionError {
    /// The multiple content blocks is empty.
    #[error("The multiple content block is empty")]
    Empty,
    /// The first content block in multiple blocks is not a text block or text delta block.
    #[error("The first content block in multiple blocks is not a text block or text delta block")]
    NotTextBlock,
}

/// The error type for parsing the image media type from an extension in a path.
#[derive(Debug, PartialEq, thiserror::Error)]
pub enum ImageMediaTypeParseError {
    /// The extension is not supported
    #[error("The extension is not supported: {0}")]
    NotSupported(String),
    /// Extension is not found
    #[error("Extension is not found")]
    NotFound,
}
