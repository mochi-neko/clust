use std::fmt::Display;

/// System prompt.
///
/// A system prompt is a way of providing context and instructions to Claude, such as specifying a particular goal or role.
/// See our [guide to system prompts](https://docs.anthropic.com/claude/docs/system-prompts).
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct SystemPrompt {
    value: String,
}

impl Default for SystemPrompt {
    fn default() -> Self {
        Self {
            value: String::new(),
        }
    }
}

impl Display for SystemPrompt {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl From<String> for SystemPrompt {
    fn from(value: String) -> Self {
        Self {
            value,
        }
    }
}

impl From<&str> for SystemPrompt {
    fn from(value: &str) -> Self {
        Self {
            value: value.to_string(),
        }
    }
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new() {
        let system_prompt = SystemPrompt::new("system-prompt");
        assert_eq!(system_prompt.value, "system-prompt");
    }

    #[test]
    fn default() {
        assert_eq!(SystemPrompt::default().value, "");
    }

    #[test]
    fn display() {
        let system_prompt = SystemPrompt::new("system-prompt");
        assert_eq!(
            system_prompt.to_string(),
            "system-prompt"
        );
    }

    #[test]
    fn serialize() {
        let system_prompt = SystemPrompt::new("system-prompt");
        assert_eq!(
            serde_json::to_string(&system_prompt).unwrap(),
            "\"system-prompt\""
        );
    }

    #[test]
    fn deserialize() {
        let system_prompt = SystemPrompt::new("system-prompt");
        assert_eq!(
            serde_json::from_str::<SystemPrompt>("\"system-prompt\"").unwrap(),
            system_prompt
        );
    }
}
