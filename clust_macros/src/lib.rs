use crate::tool::impl_tool;
use proc_macro::TokenStream;

mod check_result;
mod tool;

/// A procedural macro that generates a `clust::messages::Tool` or `clust::messages::AsyncTool`
/// implementation for the annotated function with documentation.
///
/// ## Supported arguments
/// - None
///   - e.g. `fn function() -> T`
/// - A type or types that implement(s) `std::str::FromStr`.
///   - e.g.
///     - `fn function(arg1: u32) -> T`
///     - `fn function(arg1: DefinedStruct) -> T` where `DefinedStruct` implements `std::str::FromStr`.
///
/// ## Supported return values
/// - A type that implements `std::fmt::Display`.
///   - e.g.
///     - `fn function() -> u32`
///     - `fn function() -> DefinedStruct` where `DefinedStruct` implements `std::fmt::Display`.
/// - Result<T, E> where T and E implement `std::fmt::Display`.
///   - e.g.
///     - `fn function() -> Result<u32, Error>`
///     - `fn function() -> Result<DefinedStruct, Error>` where `DefinedStruct` and `Error` implement `std::fmt::Display`.
///
/// ## Supported executions
/// - Synchronous -> implement `clust::messages::Tool`
///   - e.g. `fn function() -> T`
/// - Asynchronous -> implement `clust::messages::AsyncTool`
///   - e.g. `async fn function() -> T`
///
/// ## Supported documentation formats
/// 1. Description block for the function at the top of document.
/// 2. Arguments block for the function with
///   - header: `# Arguments`, `## Arguments`, `# Parameters` or `## Parameters`.
///   - listed items: `- `arg1` - Description for the argument` or `* `arg1` - Description for the argument`.
/// 3. Other blocks are ignored.
///
/// ```rust
/// /// Description for the function.
/// ///
/// /// ## Arguments
/// /// - `arg1` - Description for the argument.
/// fn function(arg1: i32) -> i32 { arg1 }
/// ```
///
/// ## Examples
///
/// Implement a tool by `clust_tool` for a function with documentation:
///
/// ```rust
/// use clust_macros::clust_tool;
/// use std::collections::BTreeMap;
/// use std::iter::FromIterator;
/// use clust::messages::{FunctionCalls, Invoke, Tool};
///
/// /// Increments the argument by 1.
/// ///
/// /// ## Arguments
/// /// - `value` - Target value.
/// #[clust_tool]
/// fn incrementer(value: i32) -> i32 {
///    value + 1
/// }
///
/// let tool = ClustTool_incrementer {};
///
/// let description = tool.description();
///
/// let function_calls = FunctionCalls {
///     invoke: Invoke {
///         tool_name: String::from("incrementer"),
///         parameters: BTreeMap::from_iter(vec![(
///             "value".to_string(),
///             "42".to_string(),
///         )]),
///     },
/// };
///
/// let result = tool.call(function_calls).unwrap();
/// ```
///
/// Generated XML tool description from above implementation is as follows:
///
/// ```xml
/// <tool_description>
///   <tool_name>incrementer</tool_name>
///   <description>Increments the argument by 1.</description>
///   <parameters>
///     <parameter>
///       <name>value</name>
///       <type>i32</type>
///       <description>Target value.</description>
///     </parameter>
///   </parameters>
/// </tool_description>
/// ```
///
/// This tool can be called with a function calls as following XML format:
///
/// ```xml
/// <function_calls>
///   <invoke>
///     <tool_name>incrementer</tool_name>
///     <parameters>
///         <value>42</value>
///     </parameters>
///   </invoke>
/// </function_calls>
/// ```
///
/// Calling result is as following XML format:
///
/// ```xml
/// <function_results>
///   <result>
///     <tool_name>incrementer</tool_name>
///     <stdout>43</stdout>
///   </result>
/// </function_results>
/// ```
#[proc_macro_attribute]
pub fn clust_tool(
    _attr: TokenStream,
    item: TokenStream,
) -> TokenStream {
    let item_func = syn::parse::<syn::ItemFn>(item).unwrap();
    impl_tool(&item_func)
}
