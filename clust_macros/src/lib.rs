use crate::tool::{impl_tool, impl_tool_with_result};
use proc_macro::TokenStream;

mod tool;

#[proc_macro_attribute]
pub fn clust_tool(
    attr: TokenStream,
    item: TokenStream,
) -> TokenStream {
    let item_func = syn::parse::<syn::ItemFn>(item).unwrap();
    impl_tool(&item_func)
}

#[proc_macro_attribute]
pub fn clust_tool_result(
    attr: TokenStream,
    item: TokenStream,
) -> TokenStream {
    let item_func = syn::parse::<syn::ItemFn>(item).unwrap();
    impl_tool_with_result(&item_func)
}
