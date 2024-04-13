use clust::messages::{AsyncTool, ToolUse};

use clust_macros::clust_tool;

use std::fmt::Display;

/// A function for testing.
///
/// ## Arguments
/// - `arg1` - First argument.
#[clust_tool]
async fn test_function(arg1: i32) -> Result<i32, TestError> {
    if arg1 >= 0 {
        Ok(arg1 + 1)
    } else {
        Err(TestError {
            message: "arg1 is negative".to_string(),
        })
    }
}

#[derive(Debug)]
struct TestError {
    message: String,
}

impl Display for TestError {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

#[test]
fn test_description() {
    let tool = ClustTool_test_function {};

    assert_eq!(
        tool.definition().to_string(),
        r#"{
  "name": "test_function",
  "description": "A function for testing.",
  "input_schema": {
    "description": "A function for testing.",
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

#[tokio::test]
async fn test_call() {
    let tool = ClustTool_test_function {};

    let tool_use = ToolUse::new(
        "toolu_XXXX",
        "test_function",
        serde_json::json!({"arg1": 42}),
    );

    let result = tool.call(tool_use).await.unwrap();

    assert_eq!(result.tool_use_id, "toolu_XXXX");
    assert_eq!(result.is_error, None);
    assert_eq!(result.content.unwrap().text, "43");

    let tool_use = ToolUse::new(
        "toolu_XXXX",
        "test_function",
        serde_json::json!({"arg1": -3}),
    );

    let result = tool.call(tool_use).await.unwrap();

    assert_eq!(result.tool_use_id, "toolu_XXXX");
    assert_eq!(result.is_error, Some(true));
    assert_eq!(result.content.unwrap().text, "arg1 is negative");
}
