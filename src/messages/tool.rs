use crate::macros::impl_display_for_serialize;
use crate::messages::TextContentBlock;

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

/// A tool use request.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ToolUse {
    /// The ID of the used tool.
    pub id: String,
    /// The name of the used tool.
    pub name: String,
    /// The input of the used tool.
    pub input: serde_json::Value,
}

impl Default for ToolUse {
    fn default() -> Self {
        Self {
            id: String::default(),
            name: String::default(),
            input: serde_json::Value::Null,
        }
    }
}

impl_display_for_serialize!(ToolUse);

impl ToolUse {
    /// Creates a new `ToolUse`.
    pub fn new<S, T>(
        id: S,
        name: T,
        input: serde_json::Value,
    ) -> Self
    where
        S: Into<String>,
        T: Into<String>,
    {
        Self {
            id: id.into(),
            name: name.into(),
            input,
        }
    }
}

/// A result of a tool.
#[derive(
    Debug, Clone, PartialEq, Default, serde::Serialize, serde::Deserialize,
)]
pub struct ToolResult {
    /// The id of the tool use request this is a result for.
    pub tool_use_id: String,
    /// The result of the tool, as a string (e.g. "content": "65 degrees") or list of nested content blocks (e.g. "content": [{"type": "text", "text": "65 degrees"}]\). During beta, only the text type content blocks are supported for tool_result content.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<TextContentBlock>,
}

impl_display_for_serialize!(ToolResult);

impl ToolResult {
    /// Creates a new `ToolResult`.
    pub fn new<S, T>(
        tool_use_id: S,
        content: Option<T>,
    ) -> Self
    where
        S: Into<String>,
        T: Into<TextContentBlock>,
    {
        Self {
            tool_use_id: tool_use_id.into(),
            content: content.map(Into::into),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_tool() {
        let tool = Tool::default();
        assert_eq!(tool.name, String::default());
        assert_eq!(tool.description, None);
        assert_eq!(
            tool.input_schema,
            serde_json::Value::Null
        );
    }

    #[test]
    fn display_tool() {
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
    fn serialize_tool() {
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
    fn deserialize_tool() {
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

    #[test]
    fn default_tool_use() {
        let tool_use = ToolUse::default();
        assert_eq!(tool_use.id, String::default());
        assert_eq!(tool_use.name, String::default());
        assert_eq!(tool_use.input, serde_json::Value::Null);
    }

    #[test]
    fn display_tool_use() {
        let tool_use = ToolUse {
            id: "id".to_string(),
            name: "name".to_string(),
            input: serde_json::json!({}),
        };
        assert_eq!(
            tool_use.to_string(),
            "{\n  \"id\": \"id\",\n  \"name\": \"name\",\n  \"input\": {}\n}"
        );
    }

    #[test]
    fn serialize_tool_use() {
        let tool_use = ToolUse {
            id: "id".to_string(),
            name: "name".to_string(),
            input: serde_json::json!({}),
        };
        assert_eq!(
            serde_json::to_string(&tool_use).unwrap(),
            r#"{"id":"id","name":"name","input":{}}"#
        );
    }

    #[test]
    fn deserialize_tool_use() {
        let tool_use = ToolUse {
            id: "id".to_string(),
            name: "name".to_string(),
            input: serde_json::json!({}),
        };
        assert_eq!(
            serde_json::from_str::<ToolUse>(
                r#"{"id":"id","name":"name","input":{}}"#
            )
            .unwrap(),
            tool_use
        );
    }

    #[test]
    fn new_tool_use() {
        let tool_use = ToolUse::new("id", "name", serde_json::json!({}));
        assert_eq!(tool_use.id, "id");
        assert_eq!(tool_use.name, "name");
        assert_eq!(tool_use.input, serde_json::json!({}));
    }

    #[test]
    fn default_tool_result() {
        let tool_result = ToolResult::default();
        assert_eq!(
            tool_result.tool_use_id,
            String::default()
        );
        assert_eq!(tool_result.content, None);
    }

    #[test]
    fn display_tool_result() {
        let tool_result = ToolResult {
            tool_use_id: "id".to_string(),
            content: Some(TextContentBlock::new("text")),
        };
        assert_eq!(
            tool_result.to_string(),
            "{\n  \"tool_use_id\": \"id\",\n  \"content\": {\n    \"type\": \"text\",\n    \"text\": \"text\"\n  }\n}"
        );
    }

    #[test]
    fn serialize_tool_result() {
        let tool_result = ToolResult {
            tool_use_id: "id".to_string(),
            content: Some(TextContentBlock::new("text")),
        };
        assert_eq!(
            serde_json::to_string(&tool_result).unwrap(),
            r#"{"tool_use_id":"id","content":{"type":"text","text":"text"}}"#
        );
    }

    #[test]
    fn deserialize_tool_result() {
        let tool_result = ToolResult {
            tool_use_id: "id".to_string(),
            content: Some(TextContentBlock::new("text")),
        };
        assert_eq!(
            serde_json::from_str::<ToolResult>(
                r#"{"tool_use_id":"id","content":{"type":"text","text":"text"}}"#
            )
            .unwrap(),
            tool_result
        );
    }

    #[test]
    fn new_tool_result() {
        let tool_result = ToolResult::new(
            "id",
            Some(TextContentBlock::new("text")),
        );
        assert_eq!(tool_result.tool_use_id, "id");
        assert_eq!(
            tool_result.content,
            Some(TextContentBlock::new("text"))
        );
    }
}
