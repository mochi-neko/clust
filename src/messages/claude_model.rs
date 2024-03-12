use crate::macros::impl_enum_string_serialization;
use std::fmt::Display;

/// The model that will complete your prompt.
///
/// See [models](https://docs.anthropic.com/claude/docs/models-overview) for additional details and options.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ClaudeModel {
    // Claude 3 Opus
    /// Claude 3 Opus at 2024/02/29.
    Claude3Opus20240229,
    // Claude 3 Sonnet
    /// Claude 3 Sonnet at 2024/02/29.
    Claude3Sonnet20240229,
    // Claude 3 Haiku
    // Coming soon
}

impl Default for ClaudeModel {
    fn default() -> Self {
        Self::Claude3Sonnet20240229
    }
}

impl Display for ClaudeModel {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        match self {
            | ClaudeModel::Claude3Opus20240229 => {
                write!(f, "claude-3-opus-20240229")
            },
            | ClaudeModel::Claude3Sonnet20240229 => {
                write!(f, "claude-3-sonnet-20240229")
            },
        }
    }
}

impl ClaudeModel {
    pub(crate) fn max_tokens(&self) -> u32 {
        match self {
            | ClaudeModel::Claude3Opus20240229 => 4096,
            | ClaudeModel::Claude3Sonnet20240229 => 4096,
        }
    }
}

impl_enum_string_serialization!(
    ClaudeModel,
    Claude3Opus20240229 => "claude-3-opus-20240229",
    Claude3Sonnet20240229 => "claude-3-sonnet-20240229"
);
