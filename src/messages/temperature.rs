use crate::{ValidationError, ValidationResult};
use std::fmt::Display;

/// Amount of randomness injected into the response.
///
/// Defaults to 1.0. Ranges from 0.0 to 1.0. Use temperature closer to 0.0 for analytical / multiple choice, and closer to 1.0 for creative and generative tasks.
///
/// Note that even with temperature of 0.0, the results will not be fully deterministic.
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
pub struct Temperature {
    value: f32,
}

impl Default for Temperature {
    fn default() -> Self {
        Self {
            value: 1.0,
        }
    }
}

impl Display for Temperature {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl Temperature {
    /// Creates a new temperature.
    ///
    /// ## Arguments
    /// - `value` - The value of the temperature.
    ///
    /// ## Errors
    /// It returns a validation error if the value is not in range: `[0.0, 1.0]`.
    pub fn new(value: f32) -> ValidationResult<Self, f32> {
        if value < 0.0 || value > 1.0 {
            return Err(ValidationError {
                _type: "Temperature".to_string(),
                expected: "The temperature must be in range: [0.0, 1.0]."
                    .to_string(),
                actual: value,
            });
        }

        Ok(Self {
            value,
        })
    }
}
