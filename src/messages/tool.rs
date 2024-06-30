use crate::macros::impl_display_for_serialize;
use crate::messages::{TextContentBlock, ToolCallError};
use std::future::Future;

/// A tool that can be used by assistant.
pub trait Tool {
    /// Gets the definition of the tool.
    fn definition(&self) -> ToolDefinition;
    /// Calls the tool.
    fn call(
        &self,
        tool_use: ToolUse,
    ) -> Result<ToolResult, ToolCallError>;
}

/// An asynchronous tool that can be used by assistant.
pub trait AsyncTool {
    /// Gets the definition of the tool.
    fn definition(&self) -> ToolDefinition;
    /// Calls the tool asynchronously.
    fn call(
        &self,
        tool_use: ToolUse,
    ) -> impl Future<Output = Result<ToolResult, ToolCallError>> + Send;
}

/// A tool definition that can be used by assistant.
#[derive(
    Debug, Clone, PartialEq, Default, serde::Serialize, serde::Deserialize,
)]
pub struct ToolDefinition {
    /// Name of the tool.
    pub name: String,
    /// Optional, but strongly-recommended description of the tool.
    pub description: Option<String>,
    /// JSON schema for the tool input shape that the model will produce in tool_use output content blocks.
    pub input_schema: serde_json::Value,
}

impl_display_for_serialize!(ToolDefinition);

impl ToolDefinition {
    /// Creates a new `ToolDefinition`.
    pub fn new<S, T>(
        name: S,
        description: Option<T>,
        input_schema: serde_json::Value,
    ) -> Self
    where
        S: Into<String>,
        T: Into<String>,
    {
        Self {
            name: name.into(),
            description: description.map(Into::into),
            input_schema,
        }
    }
}

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
    /// Set to true if the tool execution resulted in an error.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_error: Option<bool>,
}

impl_display_for_serialize!(ToolResult);

impl ToolResult {
    /// Creates a new `ToolResult` as a success.
    pub fn success<S, T>(
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
            is_error: None,
        }
    }

    /// Creates a new `ToolResult` as a success without content.
    pub fn success_without_content<S>(tool_use_id: S) -> Self
    where
        S: Into<String>,
    {
        Self {
            tool_use_id: tool_use_id.into(),
            content: None,
            is_error: None,
        }
    }

    /// Creates a new `ToolResult` as an error.
    pub fn error<S, T>(
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
            is_error: Some(true),
        }
    }

    /// Creates a new `ToolResult` as an error without content.
    pub fn error_without_content<S>(tool_use_id: S) -> Self
    where
        S: Into<String>,
    {
        Self {
            tool_use_id: tool_use_id.into(),
            content: None,
            is_error: Some(true),
        }
    }
}

/// A list of tools that can be called by the assistant.
pub struct ToolList {
    tools: Vec<Box<dyn Tool>>,
}

impl ToolList {
    /// Create a new tool list.
    pub fn new(tools: Vec<Box<dyn Tool>>) -> Self {
        Self {
            tools,
        }
    }

    /// List of tool definitions.
    pub fn definitions(&self) -> Vec<ToolDefinition> {
        self.tools
            .iter()
            .map(|tool| tool.definition())
            .collect::<Vec<ToolDefinition>>()
            .into()
    }

    /// Calls a tool in this list.
    pub fn call(
        &self,
        tool_use: ToolUse,
    ) -> Result<ToolResult, ToolCallError> {
        let target_name = tool_use.name.clone();

        let target_tool = self
            .tools
            .iter()
            .find(|tool| tool.definition().name == target_name)
            .ok_or_else(|| ToolCallError::ToolNotFound(target_name))?;

        target_tool.call(tool_use)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_tool_definition() {
        let tool = ToolDefinition::default();
        assert_eq!(tool.name, String::default());
        assert_eq!(tool.description, None);
        assert_eq!(
            tool.input_schema,
            serde_json::Value::Null
        );
    }

    #[test]
    fn display_tool_definition() {
        let tool = ToolDefinition {
            name: "tool".to_string(),
            description: Some("tool description".to_string()),
            input_schema: serde_json::json!({
                "properties": {
                    "arg1": {
                        "description": "First argument.",
                        "type": "integer",
                    },
                },
                "required": ["arg1"],
                "type": "object",
            }),
        };
        assert_eq!(
            tool.to_string(),
            r#"{
  "name": "tool",
  "description": "tool description",
  "input_schema": {
    "properties": {
      "arg1": {
        "description": "First argument.",
        "type": "integer"
      }
    },
    "required": [
      "arg1"
    ],
    "type": "object"
  }
}"#
        );
    }

    #[test]
    fn serialize_tool_definition() {
        let tool = ToolDefinition {
            name: "tool".to_string(),
            description: Some("tool description".to_string()),
            input_schema: serde_json::json!({
                "properties": {
                    "arg1": {
                        "description": "First argument.",
                        "type": "integer",
                    },
                },
                "required": ["arg1"],
                "type": "object",
            }),
        };
        assert_eq!(
            serde_json::to_string(&tool).unwrap(),
            r#"{"name":"tool","description":"tool description","input_schema":{"properties":{"arg1":{"description":"First argument.","type":"integer"}},"required":["arg1"],"type":"object"}}"#,
        );
    }

    #[test]
    fn deserialize_tool_definition() {
        let tool = ToolDefinition {
            name: "tool".to_string(),
            description: Some("tool description".to_string()),
            input_schema: serde_json::json!({
                "properties": {
                    "arg1": {
                        "description": "First argument.",
                        "type": "integer",
                    },
                },
                "required": ["arg1"],
                "type": "object",
            }),
        };
        assert_eq!(
            serde_json::from_str::<ToolDefinition>(
                r#"{"name":"tool","description":"tool description","input_schema":{"properties":{"arg1":{"description":"First argument.","type":"integer"}},"required":["arg1"],"type":"object"}}"#
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
            input: serde_json::json!({
                "arg1": 42,
            }),
        };
        assert_eq!(
            tool_use.to_string(),
            r#"{
  "id": "id",
  "name": "name",
  "input": {
    "arg1": 42
  }
}"#
        );
    }

    #[test]
    fn serialize_tool_use() {
        let tool_use = ToolUse {
            id: "id".to_string(),
            name: "name".to_string(),
            input: serde_json::json!({"arg1": 42}),
        };
        assert_eq!(
            serde_json::to_string(&tool_use).unwrap(),
            r#"{"id":"id","name":"name","input":{"arg1":42}}"#
        );
    }

    #[test]
    fn deserialize_tool_use() {
        let tool_use = ToolUse {
            id: "id".to_string(),
            name: "name".to_string(),
            input: serde_json::json!({"arg1": 42}),
        };
        assert_eq!(
            serde_json::from_str::<ToolUse>(
                r#"{"id":"id","name":"name","input":{"arg1":42}}"#
            )
            .unwrap(),
            tool_use
        );
    }

    #[test]
    fn new_tool_use() {
        let tool_use = ToolUse::new(
            "id",
            "name",
            serde_json::json!({"arg1": 42}),
        );
        assert_eq!(tool_use.id, "id");
        assert_eq!(tool_use.name, "name");
        assert_eq!(
            tool_use.input,
            serde_json::json!({"arg1": 42})
        );
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
            is_error: None,
        };
        assert_eq!(
            tool_result.to_string(),
            "{\n  \"tool_use_id\": \"id\",\n  \"content\": {\n    \"type\": \"text\",\n    \"text\": \"text\"\n  }\n}"
        );

        let tool_result = ToolResult {
            tool_use_id: "id".to_string(),
            content: Some(TextContentBlock::new("text")),
            is_error: Some(true),
        };
        assert_eq!(
            tool_result.to_string(),
            "{\n  \"tool_use_id\": \"id\",\n  \"content\": {\n    \"type\": \"text\",\n    \"text\": \"text\"\n  },\n  \"is_error\": true\n}"
        );
    }

    #[test]
    fn serialize_tool_result() {
        let tool_result = ToolResult {
            tool_use_id: "id".to_string(),
            content: Some(TextContentBlock::new("text")),
            is_error: None,
        };
        assert_eq!(
            serde_json::to_string(&tool_result).unwrap(),
            r#"{"tool_use_id":"id","content":{"type":"text","text":"text"}}"#
        );

        let tool_result = ToolResult {
            tool_use_id: "id".to_string(),
            content: Some(TextContentBlock::new("text")),
            is_error: Some(true),
        };
        assert_eq!(
            serde_json::to_string(&tool_result).unwrap(),
            r#"{"tool_use_id":"id","content":{"type":"text","text":"text"},"is_error":true}"#
        );
    }

    #[test]
    fn deserialize_tool_result() {
        let tool_result = ToolResult {
            tool_use_id: "id".to_string(),
            content: Some(TextContentBlock::new("text")),
            is_error: None,
        };
        assert_eq!(
            serde_json::from_str::<ToolResult>(
                r#"{"tool_use_id":"id","content":{"type":"text","text":"text"}}"#
            )
            .unwrap(),
            tool_result
        );

        let tool_result = ToolResult {
            tool_use_id: "id".to_string(),
            content: Some(TextContentBlock::new("text")),
            is_error: Some(true),
        };
        assert_eq!(
            serde_json::from_str::<ToolResult>(
                r#"{"tool_use_id":"id","content":{"type":"text","text":"text"},"is_error":true}"#
            )
            .unwrap(),
            tool_result
        );
    }

    #[test]
    fn new_tool_result() {
        let tool_result = ToolResult::success(
            "id",
            Some(TextContentBlock::new("text")),
        );
        assert_eq!(tool_result.tool_use_id, "id");
        assert_eq!(
            tool_result.content,
            Some(TextContentBlock::new("text"))
        );
        assert_eq!(tool_result.is_error, None);

        let tool_result = ToolResult::success_without_content("id");
        assert_eq!(tool_result.tool_use_id, "id");
        assert_eq!(tool_result.content, None);
        assert_eq!(tool_result.is_error, None);

        let tool_result = ToolResult::error(
            "id",
            Some(TextContentBlock::new("text")),
        );
        assert_eq!(tool_result.tool_use_id, "id");
        assert_eq!(
            tool_result.content,
            Some(TextContentBlock::new("text"))
        );
        assert_eq!(tool_result.is_error, Some(true));

        let tool_result = ToolResult::error_without_content("id");
        assert_eq!(tool_result.tool_use_id, "id");
        assert_eq!(tool_result.content, None);
        assert_eq!(tool_result.is_error, Some(true));
    }

    #[test]
    fn tool_list() {
        struct TestTool {}

        impl Tool for TestTool {
            fn definition(&self) -> ToolDefinition {
                ToolDefinition {
                    name: "test_tool".to_string(),
                    description: Some("test tool description".to_string()),
                    input_schema: serde_json::json!({
                        "properties": {
                            "arg1": {
                                "description": "First argument.",
                                "type": "integer",
                            },
                        },
                        "required": ["arg1"],
                        "type": "object",
                    }),
                }
            }

            fn call(
                &self,
                tool_use: ToolUse,
            ) -> Result<ToolResult, ToolCallError> {
                Ok(ToolResult::success(
                    tool_use.id.clone(),
                    Some("1"),
                ))
            }
        }

        let tool_use = ToolUse {
            id: "test_tool_use_id".to_string(),
            name: "test_tool".to_string(),
            input: serde_json::json!({"arg1": 42}),
        };

        let tool_list = ToolList::new(vec![Box::new(
            TestTool {},
        )]);

        let tool_result = tool_list
            .call(tool_use)
            .unwrap();
        assert_eq!(tool_result.is_error, None);
        assert_eq!(
            tool_result.tool_use_id,
            "test_tool_use_id"
        );
        assert_eq!(
            tool_result
                .content
                .unwrap()
                .text,
            "1"
        );

        let tool_use = ToolUse {
            id: "test_tool_use_id_incorrect".to_string(),
            name: "test_tool_incorrect".to_string(),
            input: serde_json::json!({"arg1": 42}),
        };

        let tool_result = tool_list.call(tool_use);
        assert!(tool_result.is_err())
    }
}
