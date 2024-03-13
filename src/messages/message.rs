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
