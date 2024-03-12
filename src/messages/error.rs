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
