extern crate proc_macro2;
extern crate quote;
extern crate syn;

use proc_macro2::TokenStream;

pub fn query(_item: TokenStream) -> TokenStream {
    "2 + 2".parse().expect("error")
}
