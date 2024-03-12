use crate::messages::MessagesError;

/// The result type for the messages API.
pub type MessagesResult<T> = Result<T, MessagesError>;
