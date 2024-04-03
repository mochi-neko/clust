use std::collections::BTreeMap;

use quick_xml::{DeError, Writer};

use crate::messages::ToolCallError;

/// Implements XML serialization and deserialization for a type.
///
/// ## Arguments
/// - `$t`: The type.
/// - `$tag`: The tag name of the root of XML element.
macro_rules! impl_xml_serialize {
    ($t:ty, $tag:expr) => {
        impl $t {
            /// Serializes the struct to an XML string.
            pub fn serialize(&self) -> Result<String, quick_xml::DeError> {
                crate::messages::function::serialize_xml(self, $tag)
            }

            /// Deserializes the struct from an XML string.
            pub fn deserialize(xml: &str) -> Result<Self, quick_xml::DeError> {
                crate::messages::function::deserialize_xml(xml)
            }
        }

        impl std::fmt::Display for $t {
            fn fmt(
                &self,
                f: &mut std::fmt::Formatter<'_>,
            ) -> std::fmt::Result {
                let xml = self
                    .serialize()
                    .map_err(|_| std::fmt::Error)?;
                write!(f, "{}", xml)
            }
        }
    };
}

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

    /// Asynchronously calls the tool with the provided function calls.
    fn call(
        &self,
        function_calls: FunctionCalls,
    ) -> impl std::future::Future<Output = Result<FunctionResults, ToolCallError>>
           + Send;
}

/// A list of tools that can be called by the assistant.
pub struct ToolList {
    tools: Vec<Box<dyn Tool>>,
}

impl ToolList {
    /// Creates a new tool list with the provided tools.
    pub fn new(tools: Vec<Box<dyn Tool>>) -> Self {
        Self {
            tools,
        }
    }

    /// Returns the list of tool descriptions.
    pub fn tools(&self) -> Tools {
        self.tools
            .iter()
            .map(|tool| tool.description())
            .collect::<Vec<ToolDescription>>()
            .into()
    }

    /// Calls the tool with the provided function calls.
    pub fn call(
        &self,
        function_calls: FunctionCalls,
    ) -> Result<FunctionResults, ToolCallError> {
        let tool_name = function_calls
            .invoke
            .tool_name
            .clone();
        let tool = self
            .tools
            .iter()
            .find(|tool| tool.description().tool_name == tool_name)
            .ok_or_else(|| ToolCallError::ToolNotFound(tool_name.clone()))?;

        tool.call(function_calls)
    }
}

// NOTE: AsyncToolList cannot be implemented because it requires async functions (= object unsafe).

/// List of tools.
///
/// ## XML example
/// ```xml
/// <tools>
///   <tool_description>
///     <tool_name>get_current_stock_price</tool_name>
///     <description>Gets the current stock price for a company. Returns float: The current stock price. Raises ValueError: if the input symbol is invalid/unknown.</description>
///     <parameters>
///       <parameter>
///         <name>symbol</name>
///         <type>string</type>
///         <description>The stock symbol of the company to get the price for.</description>
///       </parameter>
///     </parameters>
///   </tool_description>
///   <tool_description>
///     <tool_name>get_ticker_symbol</tool_name>
///     <description>Gets the stock ticker symbol for a company searched by name. Returns str: The ticker symbol for the company stock. Raises TickerNotFound: if no matching ticker symbol is found.</description>
///     <parameters>
///       <parameter>
///         <name>company_name</name>
///         <type>string</type>
///         <description>The name of the company.</description>
///       </parameter>
///     </parameters>
///   </tool_description>
/// </tools>
/// ```
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Tools {
    /// The list of tools.
    #[serde(rename = "tool_description")]
    pub inner: Vec<ToolDescription>,
}

impl From<Vec<ToolDescription>> for Tools {
    fn from(value: Vec<ToolDescription>) -> Self {
        Self {
            inner: value,
        }
    }
}

impl_xml_serialize!(Tools, "tools");

/// Description of a tool.
///
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
    /// The name of the tool.
    pub tool_name: String,
    /// The description of the tool.
    pub description: String,
    /// The parameters of the tool.
    pub parameters: Parameters,
}

impl_xml_serialize!(ToolDescription, "tool_description");

/// List of parameters of a tool.
///
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
    /// The list of parameters.
    #[serde(rename = "parameter")]
    pub inner: Vec<Parameter>,
}

impl From<Vec<Parameter>> for Parameters {
    fn from(value: Vec<Parameter>) -> Self {
        Self {
            inner: value,
        }
    }
}

/// Parameter (or argument) of a tool.
///
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
    /// The name of the parameter.
    pub name: String,
    /// The type of the parameter.
    #[serde(rename = "type")]
    pub _type: String,
    /// The description of the parameter.
    pub description: String,
}

impl_xml_serialize!(Parameter, "parameter");

/// Function calling information by the assistant.
///
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
    /// The invocation of the function.
    pub invoke: Invoke,
}

impl_xml_serialize!(FunctionCalls, "function_calls");

/// Invoking a tool with parameters.
///
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
    /// The name of the tool to invoke.
    pub tool_name: String,
    /// The parameters to invoke.
    pub parameters: BTreeMap<String, String>,
}

impl_xml_serialize!(Invoke, "invoke");

/// Result of function calling.
///
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
    /// Success result.
    #[serde(rename = "result")]
    Result(FunctionResult),
    /// Error message.
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

impl_xml_serialize!(FunctionResults, "function_results");

/// Success result of function calling.
///
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
    /// The name of the called function.
    pub tool_name: String,
    /// The standard output as the result of the function.
    pub stdout: String,
}

impl_xml_serialize!(FunctionResult, "result");

fn deserialize_xml<'de, T>(xml: &'de str) -> Result<T, DeError>
where
    T: serde::Deserialize<'de>,
{
    quick_xml::de::from_str(xml)
}

fn serialize_xml<T>(
    object: &T,
    tag_name: &str,
) -> Result<String, DeError>
where
    T: serde::Serialize,
{
    let mut writer = Writer::new_with_indent(Vec::new(), b' ', 2);
    writer.write_serializable(tag_name, object)?;

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

        let deserialized: Vector = deserialize_xml(xml).unwrap();

        assert_eq!(deserialized.values.len(), 3);
        assert_eq!(deserialized.values[0], 1);
        assert_eq!(deserialized.values[1], 2);
        assert_eq!(deserialized.values[2], 3);

        let serialized = serialize_xml(&deserialized, "vector").unwrap();

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

        let deserialized: Parameters = deserialize_xml(xml).unwrap();

        assert_eq!(deserialized.inner.len(), 1);
        let parameter_0 = deserialized.inner[0].clone();
        assert_eq!(parameter_0.name, "location");
        assert_eq!(parameter_0._type, "string");
        assert_eq!(
            parameter_0.description,
            "The city and state, e.g. San Francisco, CA"
        );

        let serialized = serialize_xml(&deserialized, "parameters").unwrap();

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

        let deserialized: Parameters = deserialize_xml(xml).unwrap();

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

        let serialized = serialize_xml(&deserialized, "parameters").unwrap();

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

        let deserialized: ToolDescription = deserialize_xml(xml).unwrap();

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

        let serialized =
            serialize_xml(&deserialized, "tool_description").unwrap();

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

        let deserialized: Parameter = deserialize_xml(xml).unwrap();

        assert_eq!(deserialized.name, "location");
        assert_eq!(deserialized._type, "string");
        assert_eq!(
            deserialized.description,
            "The city and state, e.g. San Francisco, CA"
        );

        let serialized = serialize_xml(&deserialized, "parameter").unwrap();

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

        let deserialized: FunctionCalls = deserialize_xml(xml).unwrap();

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

        let serialized =
            serialize_xml(&deserialized, "function_calls").unwrap();

        assert_eq!(serialized, xml);
    }

    #[test]
    fn test_tools() {
        let xml = r#"
<tools>
  <tool_description>
    <tool_name>get_current_stock_price</tool_name>
    <description>Gets the current stock price for a company. Returns float: The current stock price. Raises ValueError: if the input symbol is invalid/unknown.</description>
    <parameters>
      <parameter>
        <name>symbol</name>
        <type>string</type>
        <description>The stock symbol of the company to get the price for.</description>
      </parameter>
    </parameters>
  </tool_description>
  <tool_description>
    <tool_name>get_ticker_symbol</tool_name>
    <description>Gets the stock ticker symbol for a company searched by name. Returns str: The ticker symbol for the company stock. Raises TickerNotFound: if no matching ticker symbol is found.</description>
    <parameters>
      <parameter>
        <name>company_name</name>
        <type>string</type>
        <description>The name of the company.</description>
      </parameter>
    </parameters>
  </tool_description>
</tools>"#;

        let deserialized: Tools = deserialize_xml(xml).unwrap();

        assert_eq!(deserialized.inner.len(), 2);

        let tool_0 = deserialized.inner[0].clone();
        assert_eq!(
            tool_0.tool_name,
            "get_current_stock_price"
        );
        assert_eq!(
            tool_0.description,
            "Gets the current stock price for a company. Returns float: The current stock price. Raises ValueError: if the input symbol is invalid/unknown."
        );
        assert_eq!(tool_0.parameters.inner.len(), 1);
        assert_eq!(
            tool_0.parameters.inner[0].name,
            "symbol"
        );
        assert_eq!(
            tool_0.parameters.inner[0]._type,
            "string"
        );
        assert_eq!(
            tool_0.parameters.inner[0].description,
            "The stock symbol of the company to get the price for."
        );

        let tool_1 = deserialized.inner[1].clone();
        assert_eq!(tool_1.tool_name, "get_ticker_symbol");
        assert_eq!(
            tool_1.description,
            "Gets the stock ticker symbol for a company searched by name. Returns str: The ticker symbol for the company stock. Raises TickerNotFound: if no matching ticker symbol is found."
        );
        assert_eq!(tool_1.parameters.inner.len(), 1);
        assert_eq!(
            tool_1.parameters.inner[0].name,
            "company_name"
        );
        assert_eq!(
            tool_1.parameters.inner[0]._type,
            "string"
        );
        assert_eq!(
            tool_1.parameters.inner[0].description,
            "The name of the company."
        );

        let serialized = serialize_xml(&deserialized, "tools").unwrap();

        assert_eq!(serialized, xml);
    }
}
