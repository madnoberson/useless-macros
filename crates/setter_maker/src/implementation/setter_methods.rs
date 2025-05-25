use proc_macro2::{
    Span as Span2,
    TokenStream as TokenStream2,
};
use quote::quote;
use syn::{
    GenericArgument,
    Ident,
    PathArguments,
    Type,
};

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
            let with_into = field_setter_config.with_into();

            let option_inner_type = extract_option_inner_type(field_type);

            let setter_method = match option_inner_type {
                Some(inner_type) => {
                    if with_into {
                        quote! {
                            #[must_use]
                            #method_visibility fn #method_name(
                                mut self,
                                #field_name: impl Into<#inner_type>,
                            ) -> Self {
                                self.#field_name = Some(#field_name.into());
                                self
                            }
                        }
                    } else {
                        quote! {
                            #[must_use]
                            #method_visibility fn #method_name(
                                mut self,
                                #field_name: #inner_type,
                            ) -> Self {
                                self.#field_name = Some(#field_name);
                                self
                            }
                        }
                    }
                }
                None => {
                    if with_into {
                        quote! {
                            #[must_use]
                            #method_visibility fn #method_name(
                                mut self,
                                #field_name: impl Into<#field_type>,
                            ) -> Self {
                                self.#field_name = #field_name.into();
                                self
                            }
                        }
                    } else {
                        quote! {
                            #[must_use]
                            #method_visibility fn #method_name(
                                mut self,
                                #field_name: #field_type,
                            ) -> Self {
                                self.#field_name = #field_name;
                                self
                            }
                        }
                    }
                }
            };
            setter_methods.push(setter_method);
        }
    }

    setter_methods
}

fn extract_option_inner_type(field_type: &Type) -> Option<&Type> {
    if let Type::Path(type_path) = field_type {
        if type_path.path.segments.is_empty() {
            return None;
        }

        if type_path.path.segments[0].ident == "Option" {
            if let PathArguments::AngleBracketed(args) =
                &type_path.path.segments[0].arguments
            {
                if let Some(GenericArgument::Type(inner_type)) =
                    args.args.first()
                {
                    return Some(inner_type);
                }
            }
        }
    }
    None
}
