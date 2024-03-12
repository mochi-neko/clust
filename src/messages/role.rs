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
