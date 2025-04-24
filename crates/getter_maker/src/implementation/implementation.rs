use proc_macro::TokenStream as TokenStream1;
use quote::quote;
use syn::ItemStruct;

use super::{
    getter_configs::make_getter_configs,
    getter_methods::make_getter_methods,
    impl_block::make_impl_block,
};

pub fn do_make_getters(mut input: ItemStruct) -> TokenStream1 {
    let getter_configs = make_getter_configs(&mut input.fields);
    let getter_methods = make_getter_methods(getter_configs);
    let impl_block =
        make_impl_block(&input.ident, &input.generics, getter_methods);

    quote! {
        #input
        #impl_block
    }
    .into()
}
