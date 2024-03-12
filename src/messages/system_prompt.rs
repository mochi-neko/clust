/// System prompt.
///
/// A system prompt is a way of providing context and instructions to Claude, such as specifying a particular goal or role.
/// See our [guide to system prompts](https://docs.anthropic.com/claude/docs/system-prompts).
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct SystemPrompt {
    value: String,
}

impl SystemPrompt {
    /// Creates a new system prompt.
    pub fn new<S>(value: S) -> Self
    where
        S: Into<String>,
    {
        Self {
            value: value.into(),
        }
    }
}
