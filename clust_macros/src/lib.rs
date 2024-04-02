use crate::tool::impl_tool;
use proc_macro::TokenStream;

mod check_result;
mod tool;

/// A procedural macro that generates a `clust::messages::Tool` or `clust::messages::AsyncTool` implementation for the annotated function.
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
/// - Synchronous
///   - e.g. `fn function() -> T`
/// - Asynchronous
///   - e.g. `async fn function() -> T`
#[proc_macro_attribute]
pub fn clust_tool(
    _attr: TokenStream,
    item: TokenStream,
) -> TokenStream {
    let item_func = syn::parse::<syn::ItemFn>(item).unwrap();
    impl_tool(&item_func)
}
