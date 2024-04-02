use std::collections::BTreeMap;

use quick_xml::{DeError, Writer};

use crate::messages::ToolCallError;

/// Implements [`std::fmt::Display`] for a type that can be XML serialized.
///
/// ## Arguments
/// - `$t`: The type.
/// - `$tag_name`: The tag name of the root of XML element.
macro_rules! impl_display_for_serialize_xml {
    ($t:ty, $tag_name:expr) => {
        impl std::fmt::Display for $t {
            fn fmt(
                &self,
                f: &mut std::fmt::Formatter<'_>,
            ) -> std::fmt::Result {
                let xml =
                    serialize(self, $tag_name).map_err(|_| std::fmt::Error)?;
                write!(f, "{}", xml)
            }
        }
    };
}

pub(crate) use impl_display_for_serialize_xml;

/// A tool is a function that can be called by the assistant.
pub trait Tool {
    /// Returns the description of the tool.
    fn description(&self) -> ToolDescription;

    /// Calls the tool with the provided function calls.
    fn call(
        &self,
        function_calls: FunctionCalls,
    ) -> Result<FunctionResults, ToolCallError>;
}

/// A tool is an asynchronous function that can be called by the assistant.
pub trait AsyncTool {
    /// Returns the description of the tool.
    fn description(&self) -> ToolDescription;

    /// Calls the tool with the provided function calls.
    fn call(
        &self,
        function_calls: FunctionCalls,
    ) -> impl std::future::Future<Output = Result<FunctionResults, ToolCallError>>
           + Send;
}

/// ## XML example
/// ```xml
/// <tool_description>
///   <tool_name>get_weather</tool_name>
///   <description>
///     Retrieves the current weather for a specified location.
///     Returns a dictionary with two fields:
///     - temperature: float, the current temperature in Fahrenheit
///     - conditions: string, a brief description of the current weather conditions
///     Raises ValueError if the provided location cannot be found.
///   </description>
///   <parameters>
///     <parameter>
///       <name>location</name>
///       <type>string</type>
///       <description>The city and state, e.g. San Francisco, CA</description>
///     </parameter>
///   </parameters>
/// </tool_description>
/// ```
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ToolDescription {
    pub tool_name: String,
    pub description: String,
    pub parameters: Parameters,
}

impl_display_for_serialize_xml!(ToolDescription, "tool_description");

/// ## XML example
/// ```xml
/// <parameters>
///   <parameter>
///     <name>location</name>
///     <type>string</type>
///     <description>The city and state, e.g. San Francisco, CA</description>
///   </parameter>
/// </parameters>
/// ```
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Parameters {
    #[serde(rename = "parameter")]
    pub inner: Vec<Parameter>,
}

/// ## XML example
/// ```xml
/// <parameter>
///   <name>location</name>
///   <type>string</type>
///   <description>The city and state, e.g. San Francisco, CA</description>
/// </parameter>
/// ```
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Parameter {
    pub name: String,
    #[serde(rename = "type")]
    pub _type: String,
    pub description: String,
}

impl_display_for_serialize_xml!(Parameter, "parameter");

/// ## XML example
/// ```xml
/// <function_calls>
///   <invoke>
///     <tool_name>function_name</tool_name>
///     <parameters>
///       <param1>value1</param1>
///       <param2>value2</param2>
///     </parameters>
///   </invoke>
/// </function_calls>
/// ```
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct FunctionCalls {
    pub invoke: Invoke,
}

impl_display_for_serialize_xml!(FunctionCalls, "function_calls");

/// ## XML example
/// ```xml
/// <invoke>
///   <tool_name>function_name</tool_name>
///   <parameters>
///     <param1>value1</param1>
///     <param2>value2</param2>
///   </parameters>
/// </invoke>
/// ```
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Invoke {
    pub tool_name: String,
    pub parameters: BTreeMap<String, String>,
}

impl_display_for_serialize_xml!(Invoke, "invoke");

/// ## XML example
/// ```xml
/// <function_results>
///   <result>
///     <tool_name>function_name</tool_name>
///     <stdout>
///       function result goes here
///     </stdout>
///   </result>
/// </function_results>
/// ```
///
/// or
///
/// ```xml
/// <function_results>
///   <error>
///     error message goes here
///   </error>
/// </function_results>
/// ```
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum FunctionResults {
    #[serde(rename = "result")]
    Result(FunctionResult),
    #[serde(rename = "error")]
    Error(String),
}

impl From<FunctionResult> for FunctionResults {
    fn from(value: FunctionResult) -> Self {
        Self::Result(value)
    }
}

impl From<String> for FunctionResults {
    fn from(value: String) -> Self {
        Self::Error(value)
    }
}

impl_display_for_serialize_xml!(FunctionResults, "function_results");

/// ## XML example
/// ```xml
/// <result>
///   <tool_name>function_name</tool_name>
///   <stdout>
///     function result goes here
///   </stdout>
/// </result>
/// ```
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct FunctionResult {
    pub tool_name: String,
    pub stdout: String,
}

impl_display_for_serialize_xml!(FunctionResult, "result");

pub(crate) fn deserialize<'de, T>(serialized: &'de str) -> Result<T, DeError>
where
    T: serde::Deserialize<'de>,
{
    quick_xml::de::from_str(serialized)
}

pub(crate) fn serialize<T>(
    deserialized: T,
    tag_name: &str,
) -> Result<String, DeError>
where
    T: serde::Serialize,
{
    let mut writer = Writer::new_with_indent(Vec::new(), b' ', 2);
    writer.write_serializable(tag_name, &deserialized)?;

    let xml = writer.into_inner();

    let string =
        String::from_utf8(xml).map_err(|error| DeError::from(error))?;

    Ok(string)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vector() {
        let xml = r#"
<vector>
  <value>1</value>
  <value>2</value>
  <value>3</value>
</vector>"#;

        #[derive(
            Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize,
        )]
        struct Vector {
            #[serde(rename = "value")]
            values: Vec<i32>,
        }

        let deserialized: Vector = deserialize(xml).unwrap();

        assert_eq!(deserialized.values.len(), 3);
        assert_eq!(deserialized.values[0], 1);
        assert_eq!(deserialized.values[1], 2);
        assert_eq!(deserialized.values[2], 3);

        let serialized = serialize(&deserialized, "vector").unwrap();

        assert_eq!(serialized, xml);
    }

    #[test]
    fn test_parameters() {
        let xml = r#"
<parameters>
  <parameter>
    <name>location</name>
    <type>string</type>
    <description>The city and state, e.g. San Francisco, CA</description>
  </parameter>
</parameters>"#;

        let deserialized: Parameters = deserialize(xml).unwrap();

        assert_eq!(deserialized.inner.len(), 1);
        let parameter_0 = deserialized.inner[0].clone();
        assert_eq!(parameter_0.name, "location");
        assert_eq!(parameter_0._type, "string");
        assert_eq!(
            parameter_0.description,
            "The city and state, e.g. San Francisco, CA"
        );

        let serialized = serialize(&deserialized, "parameters").unwrap();

        assert_eq!(serialized, xml);
    }

    #[test]
    fn test_multi_parameters() {
        let xml = r#"
<parameters>
  <parameter>
    <name>location</name>
    <type>string</type>
    <description>The city and state, e.g. San Francisco, CA</description>
  </parameter>
  <parameter>
    <name>temperature</name>
    <type>f32</type>
    <description>The temperature at the location.</description>
  </parameter>
</parameters>"#;

        let deserialized: Parameters = deserialize(xml).unwrap();

        assert_eq!(deserialized.inner.len(), 2);

        let parameter_0 = deserialized.inner[0].clone();
        assert_eq!(parameter_0.name, "location");
        assert_eq!(parameter_0._type, "string");
        assert_eq!(
            parameter_0.description,
            "The city and state, e.g. San Francisco, CA"
        );

        let parameter_1 = deserialized.inner[1].clone();
        assert_eq!(parameter_1.name, "temperature");
        assert_eq!(parameter_1._type, "f32");
        assert_eq!(
            parameter_1.description,
            "The temperature at the location."
        );

        let serialized = serialize(&deserialized, "parameters").unwrap();

        assert_eq!(serialized, xml);
    }

    #[test]
    fn tool_description() {
        let xml = r#"
<tool_description>
  <tool_name>get_weather</tool_name>
  <description>
    Retrieves the current weather for a specified location.
    Returns a dictionary with two fields:
    - temperature: float, the current temperature in Fahrenheit
    - conditions: string, a brief description of the current weather conditions
    Raises ValueError if the provided location cannot be found.
  </description>
  <parameters>
    <parameter>
      <name>location</name>
      <type>string</type>
      <description>The city and state, e.g. San Francisco, CA</description>
    </parameter>
  </parameters>
</tool_description>"#;

        let deserialized: ToolDescription = deserialize(xml).unwrap();

        assert_eq!(deserialized.tool_name, "get_weather");
        assert_eq!(
            deserialized.description,
            "Retrieves the current weather for a specified location.\n    Returns a dictionary with two fields:\n    - temperature: float, the current temperature in Fahrenheit\n    - conditions: string, a brief description of the current weather conditions\n    Raises ValueError if the provided location cannot be found."
        );
        assert_eq!(
            deserialized
                .parameters
                .inner
                .len(),
            1
        );
        assert_eq!(
            deserialized.parameters.inner[0].name,
            "location"
        );
        assert_eq!(
            deserialized.parameters.inner[0]._type,
            "string"
        );
        assert_eq!(
            deserialized.parameters.inner[0].description,
            "The city and state, e.g. San Francisco, CA"
        );

        let serialized = serialize(&deserialized, "tool_description").unwrap();

        assert_eq!(
            serialized,
            "\n<tool_description>\n  <tool_name>get_weather</tool_name>\n  <description>Retrieves the current weather for a specified location.\n    Returns a dictionary with two fields:\n    - temperature: float, the current temperature in Fahrenheit\n    - conditions: string, a brief description of the current weather conditions\n    Raises ValueError if the provided location cannot be found.</description>\n  <parameters>\n    <parameter>\n      <name>location</name>\n      <type>string</type>\n      <description>The city and state, e.g. San Francisco, CA</description>\n    </parameter>\n  </parameters>\n</tool_description>"
        );
    }

    #[test]
    fn parameter() {
        let xml = r#"
<parameter>
  <name>location</name>
  <type>string</type>
  <description>The city and state, e.g. San Francisco, CA</description>
</parameter>"#;

        let deserialized: Parameter = deserialize(xml).unwrap();

        assert_eq!(deserialized.name, "location");
        assert_eq!(deserialized._type, "string");
        assert_eq!(
            deserialized.description,
            "The city and state, e.g. San Francisco, CA"
        );

        let serialized = serialize(&deserialized, "parameter").unwrap();

        assert_eq!(serialized, xml);
    }

    #[test]
    fn function_calls() {
        let xml = r#"
<function_calls>
  <invoke>
    <tool_name>function_name</tool_name>
    <parameters>
      <param1>value1</param1>
      <param2>value2</param2>
    </parameters>
  </invoke>
</function_calls>"#;

        let deserialized: FunctionCalls = deserialize(xml).unwrap();

        assert_eq!(
            deserialized.invoke.tool_name,
            "function_name"
        );
        assert_eq!(
            deserialized
                .invoke
                .parameters
                .len(),
            2
        );
        assert_eq!(
            deserialized
                .invoke
                .parameters
                .get("param1")
                .unwrap(),
            "value1"
        );
        assert_eq!(
            deserialized
                .invoke
                .parameters
                .get("param2")
                .unwrap(),
            "value2"
        );

        let serialized = serialize(&deserialized, "function_calls").unwrap();

        assert_eq!(serialized, xml);
    }
}
