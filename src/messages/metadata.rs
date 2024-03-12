use crate::macros::impl_display_for_serialize;
use std::fmt::Display;

/// An object describing metadata about the request.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Metadata {
    /// An external identifier for the user who is associated with the request.
    pub user_id: UserId,
}

impl_display_for_serialize!(Metadata);

/// An external identifier for the user who is associated with the request.
///
/// This should be an uuid, hash value, or other opaque identifier. Anthropic may use this id to help detect abuse.
/// Do not include any identifying information such as name, email address, or phone number.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct UserId {
    value: String,
}

impl Display for UserId {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl UserId {
    /// Creates a new user id.
    pub fn new<S>(value: S) -> Self
    where
        S: Into<String>,
    {
        Self {
            value: value.into(),
        }
    }
}
