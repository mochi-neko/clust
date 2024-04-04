use crate::messages::ClaudeModel;
use crate::ValidationError;
use std::fmt::Display;

/// The maximum number of tokens.
///
/// Note that our models may stop before reaching this maximum. This parameter only specifies the absolute maximum number of tokens to generate.
///
/// Different models have different maximum values for this parameter. See [models](https://docs.anthropic.com/claude/docs/models-overview) for details.
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    serde::Serialize,
    serde::Deserialize,
)]
#[serde(transparent)]
pub struct MaxTokens {
    value: u32,
}

impl Default for MaxTokens {
    fn default() -> Self {
        Self {
            value: 4096,
        }
    }
}

impl Display for MaxTokens {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl MaxTokens {
    /// Creates a new maximum number of tokens.
    ///
    /// ## Arguments
    /// - `value` - The value of the maximum number of tokens.
    /// - `model` - The target Claude model.
    ///
    /// ## Errors
    /// It returns a validation error if the value is greater than the maximum number of tokens for the model.
    pub fn new(
        value: u32,
        model: ClaudeModel,
    ) -> Result<MaxTokens, ValidationError<u32>> {
        if value > model.max_tokens() {
            return Err(ValidationError {
                _type: "MaxTokens".to_string(),
                expected: format!(
                    "The maximum number of tokens for the model: {} is {}.",
                    model,
                    model.max_tokens()
                ),
                actual: value,
            });
        }

        Ok(Self {
            value,
        })
    }

    /// Creates a new maximum number of tokens for the model.
    pub fn from_model(model: ClaudeModel) -> Self {
        Self {
            value: model.max_tokens(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default() {
        assert_eq!(
            MaxTokens::default(),
            MaxTokens {
                value: 4096
            }
        );
    }

    #[test]
    fn display() {
        assert_eq!(
            MaxTokens {
                value: 4096
            }
            .to_string(),
            "4096"
        );
    }

    #[test]
    fn new() {
        assert!(
            MaxTokens::new(4096, ClaudeModel::Claude3Sonnet20240229).is_ok()
        );
        assert!(
            MaxTokens::new(4097, ClaudeModel::Claude3Sonnet20240229).is_err()
        );
    }

    #[test]
    fn serialize() {
        assert_eq!(
            serde_json::to_string(&MaxTokens::default()).unwrap(),
            "4096"
        );
        assert_eq!(
            serde_json::from_str::<MaxTokens>("4096").unwrap(),
            MaxTokens::default()
        );
    }

    #[test]
    fn deserialize() {
        assert_eq!(
            serde_json::from_str::<MaxTokens>("4096").unwrap(),
            MaxTokens::default()
        );
    }
}
