use std::collections::BTreeMap;
use std::fmt::Display;

use clust::messages::{
    FunctionCalls, FunctionResults, Invoke, Tool,
};

use clust_macros::clust_tool_result;

/// A function with returning result for testing.
///
/// ## Arguments
/// - `arg1` - First argument.
///
/// ## Examples
/// ```rust
/// ```
#[clust_tool_result]
fn test_function_with_result(arg1: i32) -> Result<u32, TestError> {
    if arg1 >= 0 {
        Ok(arg1 as u32)
    } else {
        Err(TestError {
            message: "arg1 is negative".to_string(),
        })
    }
}

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
    let tool = ClustTool_test_function_with_result {};

    assert_eq!(
        tool.description().to_string(),
        r#"
<tool_description>
  <tool_name>test_function_with_result</tool_name>
  <description>A function with returning result for testing.</description>
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

#[test]
fn test_call() {
    let tool = ClustTool_test_function_with_result {};

    let function_calls = FunctionCalls {
        invoke: Invoke {
            tool_name: String::from("test_function_with_result"),
            parameters: BTreeMap::from_iter(vec![(
                "arg1".to_string(),
                "1".to_string(),
            )]),
        },
    };

    let result = tool
        .call(function_calls)
        .unwrap();

    if let FunctionResults::Result(result) = result {
        assert_eq!(
            result.tool_name,
            "test_function_with_result"
        );
        assert_eq!(result.stdout, "1");
    } else {
        panic!("Expected FunctionResults::Result");
    }

    let function_calls = FunctionCalls {
        invoke: Invoke {
            tool_name: String::from("test_function_with_result"),
            parameters: BTreeMap::from_iter(vec![(
                "arg1".to_string(),
                "-1".to_string(),
            )]),
        },
    };

    let result = tool
        .call(function_calls)
        .unwrap();

    if let FunctionResults::Error(error) = result {
        assert_eq!(
            error,
            "arg1 is negative"
        );
    } else {
        panic!("Expected FunctionResults::Error");
    }
}
