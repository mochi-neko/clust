use std::fmt::Display;

/// Only sample from the top K options for each subsequent token.
///
/// Used to remove "long tail" low probability responses. [Learn more technical details here](https://towardsdatascience.com/how-to-sample-from-language-models-682bceb97277).
///
/// Recommended for advanced use cases only. You usually only need to use temperature.
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
pub struct TopK {
    value: u32,
}

impl Default for TopK {
    fn default() -> Self {
        Self {
            value: 50,
        }
    }
}

impl Display for TopK {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl TopK {
    /// Creates a new top_k.
    pub fn new(value: u32) -> Self {
        Self {
            value,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new() {
        let top_k = TopK::new(50);
        assert_eq!(top_k.value, 50);
    }
    
    #[test]
    fn default() {
        assert_eq!(TopK::default().value, 50);
    }

    #[test]
    fn display() {
        let top_k = TopK::new(50);
        assert_eq!(
            top_k.to_string(),
            "50"
        );
    }
    
    #[test]
    fn serialize() {
        let top_k = TopK::new(50);
        assert_eq!(
            serde_json::to_string(&top_k).unwrap(),
            "50"
        );
    }
    
    #[test]
    fn deserialize() {
        let top_k = TopK::new(50);
        assert_eq!(
            serde_json::from_str::<TopK>("50").unwrap(),
            top_k
        );
    }
}
