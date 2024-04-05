use crate::macros::impl_enum_string_serialization;
use std::fmt::Display;

/// The reason that we stopped.
///
/// This may be one of the following values:
///
/// - "end_turn": the model reached a natural stopping point
/// - "max_tokens": we exceeded the requested max_tokens or the model's maximum
/// - "stop_sequence": one of your provided custom stop_sequences was generated
/// - "tool_use": Claude wants to use an external tool
///
/// Note that these values are different from those in /v1/complete, where end_turn and stop_sequence were not differentiated.
///
/// In non-streaming mode this value is always non-null. In streaming mode, it is null in the message_start event and non-null otherwise.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum StopReason {
    /// The model reached a natural stopping point.
    EndTurn,
    /// We exceeded the requested max_tokens or the model's maximum.
    MaxTokens,
    /// One of your provided custom stop_sequences was generated.
    StopSequence,
    /// Claude wants to use an external tool.
    ToolUse,
}

impl Display for StopReason {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        match self {
            | StopReason::EndTurn => {
                write!(f, "end_turn")
            },
            | StopReason::MaxTokens => {
                write!(f, "max_tokens")
            },
            | StopReason::StopSequence => {
                write!(f, "stop_sequence")
            },
            | StopReason::ToolUse => {
                write!(f, "tool_use")
            },
        }
    }
}

impl_enum_string_serialization!(
    StopReason,
    EndTurn => "end_turn",
    MaxTokens => "max_tokens",
    StopSequence => "stop_sequence",
    ToolUse => "tool_use"
);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn display() {
        assert_eq!(
            StopReason::EndTurn.to_string(),
            "end_turn"
        );
        assert_eq!(
            StopReason::MaxTokens.to_string(),
            "max_tokens"
        );
        assert_eq!(
            StopReason::StopSequence.to_string(),
            "stop_sequence"
        );
        assert_eq!(
            StopReason::ToolUse.to_string(),
            "tool_use"
        );
    }

    #[test]
    fn serialize() {
        assert_eq!(
            serde_json::to_string(&StopReason::EndTurn).unwrap(),
            "\"end_turn\""
        );
        assert_eq!(
            serde_json::to_string(&StopReason::MaxTokens).unwrap(),
            "\"max_tokens\""
        );
        assert_eq!(
            serde_json::to_string(&StopReason::StopSequence).unwrap(),
            "\"stop_sequence\""
        );
        assert_eq!(
            serde_json::to_string(&StopReason::ToolUse).unwrap(),
            "\"tool_use\""
        );
    }

    #[test]
    fn deserialize() {
        assert_eq!(
            serde_json::from_str::<StopReason>("\"end_turn\"").unwrap(),
            StopReason::EndTurn
        );
        assert_eq!(
            serde_json::from_str::<StopReason>("\"max_tokens\"").unwrap(),
            StopReason::MaxTokens
        );
        assert_eq!(
            serde_json::from_str::<StopReason>("\"stop_sequence\"").unwrap(),
            StopReason::StopSequence
        );
        assert_eq!(
            serde_json::from_str::<StopReason>("\"tool_use\"").unwrap(),
            StopReason::ToolUse
        );
    }
}
