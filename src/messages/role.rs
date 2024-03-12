use crate::macros::impl_enum_string_serialization;
use std::fmt::Display;

/// The role of the message.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Role {
    /// The user role.
    User,
    /// The assistant role.
    Assistant,
}

impl Default for Role {
    fn default() -> Self {
        Self::User
    }
}

impl Display for Role {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        match self {
            | Role::User => {
                write!(f, "user")
            },
            | Role::Assistant => {
                write!(f, "assistant")
            },
        }
    }
}

impl_enum_string_serialization!(
    Role,
    User => "user",
    Assistant => "assistant"
);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default() {
        assert_eq!(Role::default(), Role::User);
    }

    #[test]
    fn display() {
        assert_eq!(Role::User.to_string(), "user");
        assert_eq!(Role::Assistant.to_string(), "assistant");
    }

    #[test]
    fn serialize() {
        assert_eq!(
            serde_json::to_string(&Role::User).unwrap(),
            "\"user\""
        );
        assert_eq!(
            serde_json::to_string(&Role::Assistant).unwrap(),
            "\"assistant\""
        );
    }

    #[test]
    fn deserialize() {
        assert_eq!(
            serde_json::from_str::<Role>("\"user\"").unwrap(),
            Role::User
        );
        assert_eq!(
            serde_json::from_str::<Role>("\"assistant\"").unwrap(),
            Role::Assistant
        );
    }
}
