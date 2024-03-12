use crate::macros::impl_enum_string_serialization;
use std::fmt::Display;

/// The reason that we stopped.
///
/// This may be one of the following values:
///
/// "end_turn": the model reached a natural stopping point
/// "max_tokens": we exceeded the requested max_tokens or the model's maximum
/// "stop_sequence": one of your provided custom stop_sequences was generated
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
        }
    }
}

impl_enum_string_serialization!(
    StopReason,
    EndTurn => "end_turn",
    MaxTokens => "max_tokens",
    StopSequence => "stop_sequence"
);
