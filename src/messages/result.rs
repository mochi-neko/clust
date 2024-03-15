use crate::messages::{MessagesError, StreamChunk, StreamError};

/// The result type for the messages API.
pub type MessagesResult<T> = Result<T, MessagesError>;

/// The result type as stream item for the messages API.
pub type ChunkStreamResult = Result<StreamChunk, StreamError>;
