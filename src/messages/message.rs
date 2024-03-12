use crate::macros::impl_display_for_serialize;
use crate::messages::{Content, Role};

/// The input message.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Message {
    /// The role of the message.
    pub role: Role,
    /// The content of the message.
    pub content: Content,
}

impl_display_for_serialize!(Message);

impl Message {
    /// Create a new user message.
    ///
    /// ## Arguments
    /// - `content` - The content of the message.
    pub fn user<T>(content: T) -> Self 
    where
        T: Into<Content>,
    {
        Self {
            role: Role::User,
            content: content.into(),
        }
    }
    
    /// Create a new assistant message.
    /// 
    /// ## Arguments
    /// - `content` - The content of the message.
    pub fn assistant<T>(content: T) -> Self 
    where
        T: Into<Content>,
    {
        Self {
            role: Role::Assistant,
            content: content.into(),
        }
    }
}
