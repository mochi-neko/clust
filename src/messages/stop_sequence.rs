use std::fmt::Display;

/// The stop sequence.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "hash", derive(Hash))]
#[serde(transparent)]
pub struct StopSequence {
    value: String,
}

impl Display for StopSequence {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl StopSequence {
    /// Creates a new stop sequence.
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
        let stop_sequence = StopSequence::new("stop-sequence");
        assert_eq!(stop_sequence.value, "stop-sequence");
    }

    #[test]
    fn display() {
        let stop_sequence = StopSequence::new("stop-sequence");
        assert_eq!(stop_sequence.to_string(), "stop-sequence");
    }

    #[test]
    fn serialize() {
        let stop_sequence = StopSequence::new("stop-sequence");
        assert_eq!(
            serde_json::to_string(&stop_sequence).unwrap(),
            "\"stop-sequence\""
        );
    }

    #[test]
    fn deserialize() {
        let stop_sequence = StopSequence::new("stop-sequence");
        assert_eq!(
            serde_json::from_str::<StopSequence>("\"stop-sequence\"").unwrap(),
            stop_sequence
        );
    }
}
