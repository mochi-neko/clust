use std::collections::BTreeMap;
use std::fmt::Display;
use std::str::FromStr;

use clust::messages::{FunctionCalls, FunctionResults, Invoke, Tool};

use clust_macros::clust_tool;

struct TestArgument {
    value: i32,
}

impl FromStr for TestArgument {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.parse::<i32>() {
            | Ok(arg1) => Ok(TestArgument {
                value: arg1,
            }),
            | Err(_) => Err("Failed to parse TestArgument".to_string()),
        }
    }
}

struct TestResult {
    value: i32,
}

impl Display for TestResult {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

/// A function for testing.
///
/// ## Arguments
/// - `arg1` - First argument.
#[clust_tool]
fn test_function(arg1: TestArgument) -> TestResult {
    TestResult {
        value: arg1.value + 1,
    }
}

#[test]
fn test_description() {
    let tool = ClustTool_test_function {};

    assert_eq!(
        tool.description().to_string(),
        r#"
<tool_description>
  <tool_name>test_function</tool_name>
  <description>A function for testing.</description>
  <parameters>
    <parameter>
      <name>arg1</name>
      <type>TestArgument</type>
      <description>First argument.</description>
    </parameter>
  </parameters>
</tool_description>"#
    );
}

#[test]
fn test_call() {
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
        .unwrap();

    if let FunctionResults::Result(result) = result {
        assert_eq!(result.tool_name, "test_function");
        assert_eq!(result.stdout, "43");
    } else {
        panic!("Expected FunctionResults::Result");
    }
}
