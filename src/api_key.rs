use std::env::VarError;

/// The API key of the Anthropic API.
#[derive(Clone, Eq, PartialEq, Hash)]
pub struct ApiKey {
    value: String,
}

impl ApiKey {
    /// Creates a new API key.
    pub fn new<S>(value: S) -> Self
    where
        S: Into<String>,
    {
        Self {
            value: value.into(),
        }
    }

    /// Loads the API key from the environment variable: `ANTHROPIC_API_KEY`.
    pub fn from_env() -> Result<Self, VarError> {
        let value = std::env::var("ANTHROPIC_API_KEY")?;
        Ok(Self::new(value))
    }

    pub(crate) fn value(&self) -> &str {
        &self.value
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new() {
        let api_key = ApiKey::new("api-key");
        assert_eq!(api_key.value, "api-key");
    }
}
