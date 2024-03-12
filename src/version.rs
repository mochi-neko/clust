use std::fmt::Display;

/// The API version.
///
/// See also [the API reference](https://docs.anthropic.com/claude/reference/versions).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Version {
    /// 2023-01-01
    V2023_01_01,
    /// 2023-06-01
    V2023_06_01,
}

impl Default for Version {
    fn default() -> Self {
        Self::V2023_06_01
    }
}

impl Display for Version {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        match self {
            | Version::V2023_01_01 => {
                write!(f, "2023-01-01")
            },
            | Version::V2023_06_01 => {
                write!(f, "2023-06-01")
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default() {
        assert_eq!(Version::default(), Version::V2023_06_01);
    }

    #[test]
    fn display() {
        assert_eq!(
            Version::V2023_01_01.to_string(),
            "2023-01-01",
        );
        assert_eq!(
            Version::V2023_06_01.to_string(),
            "2023-06-01",
        );
    }
}