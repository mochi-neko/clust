use crate::macros::impl_display_for_serialize;

/// Billing and rate-limit usage.
///
/// Anthropic's API bills and rate-limits by token counts, as tokens represent the underlying cost to our systems.
///
/// Under the hood, the API transforms requests into a format suitable for the model. The model's output then goes through a parsing stage before becoming an API response. As a result, the token counts in usage will not match one-to-one with the exact visible content of an API request or response.
///
/// For example, output_tokens will be non-zero, even for an empty string response from Claude.
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
    Default,
    serde::Serialize,
    serde::Deserialize,
)]
pub struct Usage {
    /// The number of input tokens which were used.
    pub input_tokens: u32,
    /// The number of output tokens which were used.
    pub output_tokens: u32,
}

impl_display_for_serialize!(Usage);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serialize() {
        let usage = Usage {
            input_tokens: 1,
            output_tokens: 2,
        };
        assert_eq!(
            serde_json::to_string(&usage).unwrap(),
            r#"{"input_tokens":1,"output_tokens":2}"#
        );
    }

    #[test]
    fn deserialize() {
        let usage = Usage {
            input_tokens: 1,
            output_tokens: 2,
        };
        assert_eq!(
            serde_json::from_str::<Usage>(
                r#"{"input_tokens":1,"output_tokens":2}"#
            )
            .unwrap(),
            usage
        );
    }
}
