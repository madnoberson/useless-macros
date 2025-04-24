use proc_macro2::{
    Span as Span2,
    TokenStream as TokenStream2,
};
use quote::quote;
use syn::Ident;

use super::getter_configs::{
    GetterConfig,
    GetterRefStrategy,
    GetterVisibility,
};

pub fn make_getter_methods(
    getter_configs: Vec<GetterConfig>,
) -> Vec<TokenStream2> {
    let mut getter_methods: Vec<TokenStream2> = Vec::new();

    for getter_config in getter_configs {
        let field = getter_config.field();

        let field_name = &field.ident;
        let field_type = &field.ty;

        let method_name = Ident::new(getter_config.name(), Span2::call_site());

        let method_visibility = match getter_config.visibility() {
            GetterVisibility::Pub => Some(quote! { pub }),
            GetterVisibility::PubForCrate => Some(quote! { pub(crate) }),
            GetterVisibility::Private => None,
        };
        let reference = match getter_config.ref_strategy() {
            GetterRefStrategy::Ref => Some(quote!( & )),
            GetterRefStrategy::None => None,
            _ => unreachable!(),
        };

        let getter_method = quote! {
            #method_visibility fn #method_name(&self) -> #reference #field_type {
                #reference self.#field_name
            }
        };
        getter_methods.push(getter_method);
    }

    getter_methods
}
