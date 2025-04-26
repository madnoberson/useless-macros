use proc_macro2::{
    Span as Span2,
    TokenStream as TokenStream2,
};
use quote::quote;
use syn::Ident;

use super::setter_configs::SetterConfigs;

pub fn make_setter_methods(
    setter_configs: SetterConfigs,
) -> Vec<TokenStream2> {
    let mut setter_methods: Vec<TokenStream2> = Vec::new();

    for (field, field_setter_configs) in setter_configs.iter() {
        let field_name = &field.ident;
        let field_type = &field.ty;
        let span = Span2::call_site();

        for field_setter_config in field_setter_configs {
            let method_name = Ident::new(field_setter_config.name(), span);
            let method_visibility = field_setter_config.visibility();

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
    }

    setter_methods
}
