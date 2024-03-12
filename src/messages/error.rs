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
