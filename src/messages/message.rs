use crate::macros::impl_display_for_serialize;
use crate::messages::{Content, Role};

/// The message.
#[derive(
    Debug, Clone, PartialEq, Default, serde::Serialize, serde::Deserialize,
)]
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
    /// - `content` - The content of the user message.
    ///
    /// ## Example
    /// ```rust
    /// use clust::messages::{Content, ContentBlock, Message, Role, ImageContentSource, ImageMediaType};
    ///
    /// let text_content = "user-message";
    /// let message = Message::user(text_content); // Generics
    /// let message = Message::user(text_content.into()); // From trait
    /// let message = Message::user(Content::SingleText(text_content.to_string())); // Manual
    ///
    /// let image_content = ImageContentSource::new(
    ///     ImageMediaType::Png,
    ///     "base64 encoded image",
    /// );
    /// let message = Message::user(image_content.clone()); // Generics
    /// let message = Message::user(image_content.into()); // From trait
    /// let message = Message::user(Content::MultipleBlock(vec![ContentBlock::image(image_content.clone())])); // Manual
    /// ```
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
    /// - `content` - The content of the assistant message.
    ///
    /// ## Example
    /// ```rust
    /// use clust::messages::{Content, ContentBlock, Message};
    ///
    /// let text_content = "assistant-message";
    /// let message = Message::assistant(text_content); // Generics
    /// let message = Message::assistant(text_content.into()); // From trait
    /// let message = Message::assistant(Content::SingleText(text_content.to_string())); // Manual
    /// ```
    pub fn assistant<T>(content: T) -> Self
    where
        T: Into<Content>,
    {
        Self {
            role: Role::Assistant,
            content: content.into(),
        }
    }

    /// Create a new message.
    ///
    /// ## Arguments
    /// - `role` - The role of the message.
    /// - `content` - The content of the message.
    ///
    /// ## Example
    /// ```rust
    /// use clust::messages::{Content, ContentBlock, Message, Role, ImageContentSource, ImageMediaType};
    ///
    /// let text_content = "message";
    /// let message = Message::new(Role::User, text_content); // Generics
    /// let message = Message::new(Role::Assistant, text_content.into()); // From trait
    /// let message = Message::new(Role::Assistant, Content::SingleText(text_content.to_string())); // Manual
    ///
    /// let image_content = ImageContentSource::new(
    ///     ImageMediaType::Png,
    ///     "base64 encoded image",
    /// );
    /// let message = Message::new(Role::User, image_content.clone()); // Generics
    /// let message = Message::new(Role::User, image_content.into()); // From trait
    /// let message = Message::new(Role::User, Content::MultipleBlock(vec![ContentBlock::image(image_content.clone())])); // Manual
    /// ```
    pub fn new<T>(
        role: Role,
        content: T,
    ) -> Self
    where
        T: Into<Content>,
    {
        Self {
            role,
            content: content.into(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn user() {
        let message = Message::user("user-message");
        assert_eq!(message.role, Role::User);
        assert_eq!(message.content, "user-message".into());
    }

    #[test]
    fn assistant() {
        let message = Message::assistant("assistant-message");
        assert_eq!(message.role, Role::Assistant);
        assert_eq!(
            message.content,
            "assistant-message".into()
        );
    }

    #[test]
    fn default() {
        assert_eq!(
            Message::default(),
            Message {
                role: Role::User,
                content: Content::default(),
            }
        );
    }

    #[test]
    fn display() {
        let message = Message::user("user-message");
        assert_eq!(
            message.to_string(),
            "{\n  \"role\": \"user\",\n  \"content\": \"user-message\"\n}"
        );
    }

    #[test]
    fn serialize() {
        let message = Message::user("user-message");
        assert_eq!(
            serde_json::to_string(&message).unwrap(),
            "{\"role\":\"user\",\"content\":\"user-message\"}"
        );
    }

    #[test]
    fn deserialize() {
        let message = Message::user("user-message");
        assert_eq!(
            serde_json::from_str::<Message>(
                "{\"role\":\"user\",\"content\":\"user-message\"}"
            )
            .unwrap(),
            message
        );
    }
}
