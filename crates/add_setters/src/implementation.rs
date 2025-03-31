use proc_macro::TokenStream as TokenStream1;
use proc_macro2::{Span as Span2, TokenStream as TokenStream2};
use quote::quote;
use syn::{Field, Fields, Generics, Ident, ItemStruct};

pub fn do_add_setters(mut input: ItemStruct) -> TokenStream1 {
    let fields = fields_to_make_setters_for(&mut input.fields);
    let setter_methods = make_setter_methods(fields);
    let impl_block = make_impl_block(&input.ident, &input.generics, setter_methods);
    expand_code(&input, impl_block)
}

fn fields_to_make_setters_for(fields: &mut Fields) -> Vec<&Field> {
    let unfiltered_fields = match fields {
        Fields::Named(fields) => &mut fields.named,
        _ => panic!("`add_setters` only supports structs with named fields."),
    };
    let mut fields_to_make_setters_for: Vec<&Field> = Vec::new();

    for field in unfiltered_fields {
        let field_requires_setter = !field
            .attrs
            .iter()
            .any(|attr| attr.path().is_ident("disable_setter"));

        if field_requires_setter {
            fields_to_make_setters_for.push(field);
            continue;
        }
        field
            .attrs
            .retain(|attr| !attr.path().is_ident("disable_setter"));
    }

    fields_to_make_setters_for
}

fn make_setter_methods(fields: Vec<&Field>) -> Vec<TokenStream2> {
    let mut setter_methods: Vec<TokenStream2> = Vec::new();

    for field in fields {
        let field_name = field.ident.as_ref().unwrap();
        let field_type = &field.ty;

        let method_name = Ident::new(&format!("with_{field_name}"), Span2::call_site());

        let setter_method = quote! {
            #[must_use]
            pub fn #method_name(
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

fn make_impl_block(
    struct_name: &Ident,
    struct_genertic: &Generics,
    setter_methods: Vec<TokenStream2>,
) -> TokenStream2 {
    let setter_methods: TokenStream2 = setter_methods.into_iter().collect();
    let (impl_generics, type_generics, where_clause) = struct_genertic.split_for_impl();

    quote! {
        impl #impl_generics #struct_name #type_generics #where_clause {
            #setter_methods
        }
    }
}

fn expand_code(input: &ItemStruct, impl_block: TokenStream2) -> TokenStream1 {
    quote! {
        #input
        #impl_block
    }
    .into()
}
