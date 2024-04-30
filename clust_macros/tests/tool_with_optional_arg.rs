use clust::messages::{Tool, ToolUse};

use clust_macros::clust_tool;

/// A function for testing.
///
/// ## Arguments
/// - `arg1` - First argument.
#[clust_tool]
fn test_function(arg1: Option<i32>) -> i32 {
    if let Some(arg1) = arg1 {
        arg1
    } else {
        0
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
    "required": [],
    "type": "object"
  }
}"#
    );
}

#[test]
fn test_call() {
    let tool = ClustTool_test_function {};

    let tool_use = ToolUse::new(
        "toolu_XXXX",
        "test_function",
        serde_json::json!({}),
    );

    let result = tool.call(tool_use).unwrap();

    assert_eq!(result.tool_use_id, "toolu_XXXX");
    assert_eq!(result.is_error, None);
    assert_eq!(result.content.unwrap().text, "0");

    let tool_use = ToolUse::new(
        "toolu_XXXX",
        "test_function",
        serde_json::json!({
            "arg1": 42
        }),
    );

    let result = tool.call(tool_use).unwrap();

    assert_eq!(result.tool_use_id, "toolu_XXXX");
    assert_eq!(result.is_error, None);
    assert_eq!(result.content.unwrap().text, "42");
}
