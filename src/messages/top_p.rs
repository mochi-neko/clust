use crate::{ValidationError, ValidationResult};
use std::fmt::Display;

/// Use nucleus sampling.
///
/// In nucleus sampling, we compute the cumulative distribution over all the options for each subsequent token in decreasing probability order and cut it off once it reaches a particular probability specified by top_p. You should either alter temperature or top_p, but not both.
///
/// Recommended for advanced use cases only. You usually only need to use temperature.
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    PartialOrd,
    serde::Serialize,
    serde::Deserialize,
)]
#[serde(transparent)]
pub struct TopP {
    value: f32,
}

impl Default for TopP {
    fn default() -> Self {
        Self {
            value: 1.0,
        }
    }
}

impl Display for TopP {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl TopP {
    /// Creates a new top_p.
    ///
    /// ## Arguments
    /// - `value` - The value of the top_p.
    ///
    /// ## Errors
    /// It returns a validation error if the value is not in range: `[0.0, 1.0]`.
    pub fn new(value: f32) -> ValidationResult<Self, f32> {
        if value < 0.0 || value > 1.0 {
            return Err(ValidationError {
                _type: "TopP".to_string(),
                expected: "The top_p must be in range: [0.0, 1.0].".to_string(),
                actual: value,
            });
        }

        Ok(Self {
            value,
        })
    }
}
