use std::collections::BTreeMap;

use clust::messages::{AsyncTool, FunctionCalls, FunctionResults, Invoke};

use clust_macros::clust_tool;

/// An asynchronous function for testing.
///
/// ## Arguments
/// - `arg1` - First argument.
#[clust_tool]
async fn test_function(arg1: i32) -> i32 {
    arg1 + 1
}

#[test]
fn test_description() {
    let tool = ClustTool_test_function {};

    assert_eq!(
        tool.description().to_string(),
        r#"
<tool_description>
  <tool_name>test_function</tool_name>
  <description>An asynchronous function for testing.</description>
  <parameters>
    <parameter>
      <name>arg1</name>
      <type>i32</type>
      <description>First argument.</description>
    </parameter>
  </parameters>
</tool_description>"#
    );
}

#[tokio::test]
async fn test_call() {
    let tool = ClustTool_test_function {};

    let function_calls = FunctionCalls {
        invoke: Invoke {
            tool_name: String::from("test_function"),
            parameters: BTreeMap::from_iter(vec![(
                "arg1".to_string(),
                "42".to_string(),
            )]),
        },
    };

    let result = tool
        .call(function_calls)
        .await
        .unwrap();

    if let FunctionResults::Result(result) = result {
        assert_eq!(result.tool_name, "test_function");
        assert_eq!(result.stdout, "43");
    } else {
        panic!("Expected FunctionResults::Result");
    }
}
