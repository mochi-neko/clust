use std::collections::BTreeMap;

use clust::messages::{
    FunctionCalls, FunctionResult, FunctionResults, Invoke, Tool,
};

use clust_macros::clust_tool;

/// A function with multiple arguments for testing.
///
/// ## Arguments
/// - `arg1` - First argument.
/// - `arg2` - Second argument.
#[clust_tool]
fn test_function_multi_args(
    arg1: u32,
    arg2: u32,
) -> u32 {
    arg1 + arg2
}

#[test]
fn test_description() {
    let tool = ClustTool_test_function_multi_args {};

    assert_eq!(
        tool.description().to_string(),
        r#"
<tool_description>
  <tool_name>test_function_multi_args</tool_name>
  <description>A function with multiple arguments for testing.</description>
  <parameters>
    <parameter>
      <name>arg1</name>
      <type>u32</type>
      <description>First argument.</description>
    </parameter>
    <parameter>
      <name>arg2</name>
      <type>u32</type>
      <description>Second argument.</description>
    </parameter>
  </parameters>
</tool_description>"#
    );
}

#[test]
fn test_call() {
    let tool = ClustTool_test_function_multi_args {};

    let function_calls = FunctionCalls {
        invoke: Invoke {
            tool_name: String::from("test_function_multi_args"),
            parameters: BTreeMap::from_iter(vec![
                ("arg1".to_string(), "42".to_string()),
                ("arg2".to_string(), "21".to_string()),
            ]),
        },
    };

    let result = tool
        .call(function_calls)
        .unwrap();

    if let FunctionResults::Result(result) = result {
        assert_eq!(
            result.tool_name,
            "test_function_multi_args"
        );
        assert_eq!(result.stdout, "63");
    } else {
        panic!("Expected FunctionResults::Result");
    }
}
