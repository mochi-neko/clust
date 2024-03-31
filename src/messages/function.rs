use quick_xml::{DeError, Writer};
use std::collections::BTreeMap;

/// A tool is a function that can be called by the assistant.
pub trait Tool {
    /// Returns the description of the tool.
    fn description(&self) -> ToolDescription;

    /// Calls the tool with the provided function calls.
    fn call(
        &self,
        function_calls: FunctionCalls,
    ) -> FunctionResults;
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
    pub parameters: Vec<ParameterElement>,
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
pub struct ParameterElement {
    pub parameter: Parameter,
}

/// ## XML example
/// ```xml
/// <name>location</name>
/// <type>string</type>
/// <description>The city and state, e.g. San Francisco, CA</description>
/// ```
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Parameter {
    pub name: String,
    #[serde(rename = "type")]
    pub _type: String,
    pub description: String,
}

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

pub(crate) fn deserialize<'de, T>(
    serialized: &'de str
) -> std::result::Result<T, DeError>
where
    T: serde::Deserialize<'de>,
{
    quick_xml::de::from_str(serialized)
}

pub(crate) fn serialize<T>(
    deserialized: T,
    tag_name: &str,
) -> std::result::Result<String, DeError>
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
<parameters>
  <parameter>
    <name>location</name>
    <type>string</type>
    <description>The city and state, e.g. San Francisco, CA</description>
  </parameter>
</parameters>"#;

        let deserialized: Vec<ParameterElement> = deserialize(xml).unwrap();

        assert_eq!(deserialized.len(), 1);
        assert_eq!(
            deserialized[0].parameter.name,
            "location"
        );
        assert_eq!(
            deserialized[0]
                .parameter
                ._type,
            "string"
        );
        assert_eq!(
            deserialized[0]
                .parameter
                .description,
            "The city and state, e.g. San Francisco, CA"
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
        assert_eq!(deserialized.parameters.len(), 1);
        assert_eq!(
            deserialized.parameters[0]
                .parameter
                .name,
            "location"
        );
        assert_eq!(
            deserialized.parameters[0]
                .parameter
                ._type,
            "string"
        );
        assert_eq!(
            deserialized.parameters[0]
                .parameter
                .description,
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
