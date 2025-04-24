use proc_macro::TokenStream;
use syn::{
    ItemStruct,
    parse_macro_input,
};

mod implementation;
use implementation::do_make_getters;

#[proc_macro_attribute]
pub fn make_getters(_: TokenStream, input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as ItemStruct);
    do_make_getters(input)
}
