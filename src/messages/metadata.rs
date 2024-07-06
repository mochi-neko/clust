use crate::macros::impl_display_for_serialize;
use std::fmt::Display;

/// An object describing metadata about the request.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "hash", derive(Hash))]
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
#[cfg_attr(feature = "hash", derive(Hash))]
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

impl From<&str> for UserId {
    fn from(value: &str) -> Self {
        Self {
            value: value.to_string(),
        }
    }
}

impl From<String> for UserId {
    fn from(value: String) -> Self {
        Self {
            value,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_metadata() {
        let metadata = Metadata {
            user_id: UserId::new("user-id"),
        };
        assert_eq!(metadata.user_id.value, "user-id");
    }

    #[test]
    fn display_metadata() {
        let metadata = Metadata {
            user_id: UserId::new("user-id"),
        };
        assert_eq!(
            metadata.to_string(),
            "{\n  \"user_id\": \"user-id\"\n}"
        );
    }

    #[test]
    fn serialize_metadata() {
        let metadata = Metadata {
            user_id: UserId::new("user-id"),
        };
        assert_eq!(
            serde_json::to_string(&metadata).unwrap(),
            "{\"user_id\":\"user-id\"}"
        );
    }

    #[test]
    fn deserialize_metadata() {
        let metadata = Metadata {
            user_id: UserId::new("user-id"),
        };
        assert_eq!(
            serde_json::from_str::<Metadata>("{\"user_id\":\"user-id\"}")
                .unwrap(),
            metadata
        );
    }

    #[test]
    fn new_user_id() {
        let user_id = UserId::new("user-id");
        assert_eq!(user_id.value, "user-id");
    }

    #[test]
    fn from_str_user_id() {
        let user_id = UserId::from("user-id");
        assert_eq!(user_id.value, "user-id");
    }

    #[test]
    fn from_string_user_id() {
        let user_id = UserId::from("user-id".to_string());
        assert_eq!(user_id.value, "user-id");
    }

    #[test]
    fn display_user_id() {
        let user_id = UserId::new("user-id");
        assert_eq!(user_id.to_string(), "user-id");
    }

    #[test]
    fn serialize_user_id() {
        let user_id = UserId::new("user-id");
        assert_eq!(
            serde_json::to_string(&user_id).unwrap(),
            "\"user-id\""
        );
    }

    #[test]
    fn deserialize_user_id() {
        let user_id = UserId::new("user-id");
        assert_eq!(
            serde_json::from_str::<UserId>("\"user-id\"").unwrap(),
            user_id
        );
    }
}
