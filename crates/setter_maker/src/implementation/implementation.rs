use proc_macro::TokenStream as TokenStream1;
use quote::quote;
use syn::ItemStruct;

use super::{
    impl_block::make_impl_block,
    setter_configs::make_setter_configs,
    setter_methods::make_setter_methods,
};

pub fn do_make_setters(mut input: ItemStruct) -> TokenStream1 {
    let setter_configs = make_setter_configs(&mut input.fields);
    let setter_methods = make_setter_methods(setter_configs);
    let impl_block =
        make_impl_block(&input.ident, &input.generics, setter_methods);

    quote! {
        #input
        #impl_block
    }
    .into()
}
