use crate::{ApiError, ClientError};
use std::fmt::Display;

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
    #[error("Stream option mismatch")]
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
    /// Chunk type error.
    #[error(transparent)]
    MessageChunkTypeError(#[from] MessageChunkTypeError),
    /// Parse chunk string error.
    #[error("Parse chunk string error: {0}")]
    ParseChunkStringError(String),
    /// Chunk data deserialization error.
    #[error(transparent)]
    ChunkDataDeserializationError(#[from] serde_json::Error),
}

/// The error type for parsing message chunk type.
#[derive(Debug, thiserror::Error)]
pub struct MessageChunkTypeError {
    /// The actual chunk type.
    pub chunk_type: String,
}

impl Display for MessageChunkTypeError {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        write!(
            f,
            "Not supported message chunk type: {}",
            self.chunk_type
        )
    }
}

/// The error type for the content flattening.
#[derive(Debug, PartialEq, thiserror::Error)]
pub enum ContentFlatteningError {
    /// The multiple content blocks is empty.
    #[error("The multiple content block is empty")]
    Empty,
    /// Not found target block.
    #[error("Not found target block")]
    NotFoundTargetBlock,
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
