//! Provides procedural macros for the [clust](https://github.com/mochi-neko/clust).

use crate::tool::impl_tool;
use proc_macro::TokenStream;

mod parameter_type;
mod return_type;
mod tool;

/// A procedural macro that generates a `clust::messages::Tool` or `clust::messages::AsyncTool`
/// implementation for the annotated function with documentation.
///
/// See also [the official guide](https://docs.anthropic.com/claude/docs/functions-external-tools).
///
/// ## Supported arguments
/// - None
///   - e.g. `fn function() -> T`
/// - Types that can be represented as JSON object.
///   - Boolean
///     - `bool`
///   - Integer
///     - `i8`, `i16`, `i32`, `i64`, `i128`
///     - `u8`, `u16`, `u32`, `u64`, `u128`
///   - Number
///     - `f32`
///     - `f64`
///   - String
///     - `String`
///     - `&str`
///   - Array
///     - `Vec<T>` where `T` is supported type.
///     - `&[T]` where `T` is supported type.
///     - `&[T; N]` where `T` is supported type and `N` is a constant.
///   - Option
///     - `Option<T>` where `T` is supported type.
///   - e.g. `fn function(arg1: i32, arg2: String, arg3: Vec<f64>) -> T`
///
/// ## Supported return values
/// - None
///   - e.g. `fn function()`
/// - A type that can be formatted, i.e. implements `std::fmt::Display`.
///   - e.g.
///     - `fn function() -> u32`
///     - `fn function() -> DefinedStruct` (where `DefinedStruct` implements `std::fmt::Display`).
/// - Result<T, E> where T and E can be formatted, i.e. implement `std::fmt::Display`.
///   - e.g.
///     - `fn function() -> Result<u32, SomeError>` (where `SomeError` implements `std::fmt::Display`).
///     - `fn function() -> Result<DefinedStruct, SomeError>` (where `DefinedStruct` and `SomeError` implement `std::fmt::Display`).
///
/// ## Supported executions
/// - Synchronous -> implement `clust::messages::Tool`
///   - e.g. `fn function() -> T`
/// - Asynchronous -> implement `clust::messages::AsyncTool`
///   - e.g. `async fn function() -> T`
///
/// ## (Optional) Supported documentation formats
/// 1. Description block for the function at the top of document.
/// 2. Arguments block for the function with
///   - header: `# Arguments`, `## Arguments`, `# Parameters` or `## Parameters`.
///   - listed items: `- `arg1` - Description for the argument` or `* `arg1` - Description for the argument`.
/// 3. Other blocks are ignored.
///
/// e.g.
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
/// use clust::messages::{ToolUse, Tool};
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
/// let tool_use = ToolUse::new(
///     "toolu_XXXX",
///     "incrementer",
///     serde_json::json!({
///         "value": 42
///     }),
/// );
///
/// let result = tool.call(tool_use).unwrap();
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
