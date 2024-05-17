extern crate proc_macro2;
extern crate proc_macro;
extern crate quote;
extern crate syn;
extern crate rsqlite_macros_internals as rsqli;


use proc_macro::TokenStream;

#[proc_macro]
pub fn query(item: TokenStream) -> TokenStream {
    rsqli::query(item.into()).into()
}
