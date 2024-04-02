use crate::tool::impl_tool;
use proc_macro::TokenStream;

mod check_result;
mod tool;

#[proc_macro_attribute]
pub fn clust_tool(
    _attr: TokenStream,
    item: TokenStream,
) -> TokenStream {
    let item_func = syn::parse::<syn::ItemFn>(item).unwrap();
    impl_tool(&item_func)
}
