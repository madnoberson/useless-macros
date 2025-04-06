use proc_macro2::{
    Span as Span2,
    TokenStream as TokenStream2,
};
use quote::quote;
use syn::Ident;

use super::setter_configs::{
    SetterConfig,
    SetterVisibility,
};

pub fn make_setter_methods(
    setter_configs: Vec<SetterConfig>,
) -> Vec<TokenStream2> {
    let mut setter_methods: Vec<TokenStream2> = Vec::new();

    for setter_config in setter_configs {
        let field = setter_config.field();

        let field_name = &field.ident;
        let field_type = &field.ty;

        let method_name = Ident::new(setter_config.name(), Span2::call_site());

        let method_visibility = match setter_config.visibility() {
            SetterVisibility::Pub => Some(quote! { pub }),
            SetterVisibility::PubForCrate => Some(quote! { pub(crate) }),
            SetterVisibility::Private => None,
        };

        let setter_method = quote! {
            #[must_use]
            #method_visibility fn #method_name(
                mut self,
                #field_name: impl Into<#field_type>,
            ) -> Self {
                self.#field_name = #field_name.into();
                self
            }
        };
        setter_methods.push(setter_method);
    }

    setter_methods
}
