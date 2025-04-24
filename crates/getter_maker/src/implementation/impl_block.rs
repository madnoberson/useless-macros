use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::{
    Generics,
    Ident,
};

pub fn make_impl_block(
    struct_name: &Ident,
    struct_genertic: &Generics,
    getter_methods: Vec<TokenStream2>,
) -> TokenStream2 {
    let getter_methods: TokenStream2 = getter_methods.into_iter().collect();
    let (impl_generics, type_generics, where_clause) =
        struct_genertic.split_for_impl();

    quote! {
        impl #impl_generics #struct_name #type_generics #where_clause {
            #getter_methods
        }
    }
}
