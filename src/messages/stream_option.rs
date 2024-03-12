use crate::macros::impl_enum_bool_serialization;
use std::fmt::Display;

/// Whether to incrementally stream the response using server-sent events.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum StreamOption {
    /// "false": Returns the response once.
    ReturnOnce,
    /// "true": Returns the response in a stream.
    ReturnStream,
}

impl Default for StreamOption {
    fn default() -> Self {
        Self::ReturnOnce
    }
}

impl Display for StreamOption {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        match self {
            | StreamOption::ReturnOnce => {
                write!(f, "false")
            },
            | StreamOption::ReturnStream => {
                write!(f, "true")
            },
        }
    }
}

impl From<bool> for StreamOption {
    fn from(value: bool) -> Self {
        if value {
            Self::ReturnStream
        } else {
            Self::ReturnOnce
        }
    }
}

impl_enum_bool_serialization!(StreamOption, ReturnStream, ReturnOnce);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default() {
        assert_eq!(
            StreamOption::default(),
            StreamOption::ReturnOnce
        );
    }

    #[test]
    fn display() {
        assert_eq!(
            StreamOption::ReturnOnce.to_string(),
            "false"
        );
        assert_eq!(
            StreamOption::ReturnStream.to_string(),
            "true"
        );
    }

    #[test]
    fn from_bool() {
        assert_eq!(
            StreamOption::from(false),
            StreamOption::ReturnOnce
        );
        assert_eq!(
            StreamOption::from(true),
            StreamOption::ReturnStream
        );
    }

    #[test]
    fn serialize() {
        assert_eq!(
            serde_json::to_string(&StreamOption::ReturnOnce).unwrap(),
            "false"
        );
        assert_eq!(
            serde_json::to_string(&StreamOption::ReturnStream).unwrap(),
            "true"
        );
    }

    #[test]
    fn deserialize() {
        assert_eq!(
            serde_json::from_str::<StreamOption>("false").unwrap(),
            StreamOption::ReturnOnce
        );
        assert_eq!(
            serde_json::from_str::<StreamOption>("true").unwrap(),
            StreamOption::ReturnStream
        );
    }
}
