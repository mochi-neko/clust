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

impl_enum_bool_serialization!(StreamOption, ReturnOnce, ReturnStream);
