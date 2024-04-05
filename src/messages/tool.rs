use crate::macros::impl_display_for_serialize;

/// A tool that can be used by assistant.
#[derive(
    Debug, Clone, PartialEq, Default, serde::Serialize, serde::Deserialize,
)]
pub struct Tool {
    /// Name of the tool.
    pub name: String,
    /// Optional, but strongly-recommended description of the tool.
    pub description: Option<String>,
    /// JSON schema for the tool input shape that the model will produce in tool_use output content blocks.
    pub input_schema: serde_json::Value,
}

impl_display_for_serialize!(Tool);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default() {
        let tool = Tool::default();
        assert_eq!(tool.name, String::default());
        assert_eq!(tool.description, None);
        assert_eq!(
            tool.input_schema,
            serde_json::Value::Null
        );
    }

    #[test]
    fn display() {
        let tool = Tool {
            name: "tool".to_string(),
            description: Some("tool description".to_string()),
            input_schema: serde_json::json!({}),
        };
        assert_eq!(
            tool.to_string(),
            "{\n  \"name\": \"tool\",\n  \"description\": \"tool description\",\n  \"input_schema\": {}\n}"
        );
    }

    #[test]
    fn serialize() {
        let tool = Tool {
            name: "tool".to_string(),
            description: Some("tool description".to_string()),
            input_schema: serde_json::json!({}),
        };
        assert_eq!(
            serde_json::to_string(&tool).unwrap(),
            r#"{"name":"tool","description":"tool description","input_schema":{}}"#
        );
    }

    #[test]
    fn deserialize() {
        let tool = Tool {
            name: "tool".to_string(),
            description: Some("tool description".to_string()),
            input_schema: serde_json::json!({}),
        };
        assert_eq!(
            serde_json::from_str::<Tool>(
                r#"{"name":"tool","description":"tool description","input_schema":{}}"#
            )
            .unwrap(),
            tool
        );
    }
}
