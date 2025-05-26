use std::collections::HashMap;
use std::panic;

use proc_macro2::Span as Span2;
use syn::{
    Attribute,
    Expr,
    Field,
    Fields,
    Ident,
    Lit,
    MetaNameValue,
    Path,
    Token,
    Visibility,
    parse_str,
    punctuated::Punctuated,
};

const CONFIG_ATTRIBUTE: &str = "basic_setter";
const DISABLE_ATTRIBUTE: &str = "disable_basic_setters";

const NAME_PARAM: &str = "name";
const PREFIX_PARAM: &str = "prefix";
const SUFFIX_PARAM: &str = "suffix";
const VISIBILITY_PARAM: &str = "visibility";
const WITH_INTO_PARAM: &str = "with_into";

const DEFAULT_PREFIX: &str = "set";

pub type SetterConfigs<'a> = HashMap<&'a Field, Vec<SetterConfig>>;

#[derive(Debug)]
pub struct SetterConfig {
    name: String,
    visibility: Visibility,
    with_into: bool,
}

impl SetterConfig {
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn visibility(&self) -> &Visibility {
        &self.visibility
    }

    pub fn with_into(&self) -> bool {
        self.with_into
    }
}

pub fn make_setter_configs(fields: &mut Fields) -> SetterConfigs {
    let fields = match fields {
        Fields::Named(fields) => &mut fields.named,
        _ => panic!("Macro supports only structs with named fields."),
    };
    let mut setter_configs: SetterConfigs = SetterConfigs::new();

    for field in fields {
        let is_disabled = field
            .attrs
            .iter()
            .any(|attr| attr.path().is_ident(DISABLE_ATTRIBUTE));

        if is_disabled {
            remove_attributes(field);
            continue;
        }

        let field_setter_configs = extract_configs(field);
        remove_attributes(field);
        setter_configs.insert(field, field_setter_configs);
    }

    setter_configs
}

/// Extracts configs from config attributes of field and returns
/// them. If field has no attribute, this function returns Vec
/// with a default config.
fn extract_configs(field: &Field) -> Vec<SetterConfig> {
    let field_ident = field.ident.as_ref().unwrap();

    let attributes: Vec<&Attribute> = field
        .attrs
        .iter()
        .filter(|attr| attr.path().is_ident(CONFIG_ATTRIBUTE))
        .collect();

    if attributes.is_empty() {
        let name = format!("{DEFAULT_PREFIX}_{field_ident}");
        let visibility = default_visibility_factory();
        let with_into = true;

        return vec![SetterConfig {
            name,
            visibility,
            with_into,
        }];
    }

    let mut setter_configs: Vec<SetterConfig> = Vec::new();
    for attribute in attributes {
        let setter_config = extract_config(field_ident, attribute);
        setter_configs.push(setter_config);
    }

    setter_configs
}

fn extract_config(field_ident: &Ident, attribute: &Attribute) -> SetterConfig {
    let name_values: Punctuated<MetaNameValue, Token![,]> = attribute
        .parse_args_with(Punctuated::parse_terminated)
        .unwrap();

    let mut name: Option<String> = None;
    let mut prefix: Option<String> = None;
    let mut suffix: Option<String> = None;
    let mut raw_visibility: Option<String> = None;
    let mut with_into: Option<bool> = None;

    for name_value in name_values {
        if let Expr::Lit(lit_expr) = name_value.value {
            parse_attribute_param(
                name_value.path,
                lit_expr.lit,
                &mut name,
                &mut prefix,
                &mut suffix,
                &mut raw_visibility,
                &mut with_into,
            );
        }
    }

    let name = make_name(name, prefix, suffix, field_ident);
    let visibility = match raw_visibility.as_ref() {
        Some(raw_visibility) => parse_str(raw_visibility).unwrap(),
        None => default_visibility_factory(),
    };
    let with_into = with_into.unwrap_or(true);

    SetterConfig {
        name,
        visibility,
        with_into,
    }
}

fn parse_attribute_param(
    param_path: Path,
    param_value: Lit,
    name: &mut Option<String>,
    prefix: &mut Option<String>,
    suffix: &mut Option<String>,
    raw_visibility: &mut Option<String>,
    with_into: &mut Option<bool>,
) {
    let param_name = param_path.get_ident().unwrap().to_string();

    match param_value {
        Lit::Str(param_value) => {
            match param_name.as_str() {
                NAME_PARAM => name.insert(param_value.value()),
                PREFIX_PARAM => prefix.insert(param_value.value()),
                SUFFIX_PARAM => suffix.insert(param_value.value()),
                VISIBILITY_PARAM => raw_visibility.insert(param_value.value()),
                _ => panic!("Unexpected param."),
            };
        }
        Lit::Bool(param_value) => {
            match param_name.as_str() {
                WITH_INTO_PARAM => with_into.insert(param_value.value()),
                _ => panic!("Unexpected param."),
            };
        }
        _ => panic!("Unexpected value type."),
    };
}

fn make_name(
    name: Option<String>,
    prefix: Option<String>,
    suffix: Option<String>,
    field_ident: &Ident,
) -> String {
    let field_name = field_ident.to_string();

    match (name, prefix, suffix) {
        (None, Some(prefix), Some(suffix)) => format!("{prefix}_{suffix}"),
        (None, Some(prefix), None) => format!("{prefix}_{field_name}"),
        (None, None, Some(suffix)) => format!("{DEFAULT_PREFIX}_{suffix}"),
        (None, None, None) => format!("{DEFAULT_PREFIX}_{field_name}"),
        (Some(_), Some(_), Some(_)) => panic!(
            "'{NAME_PARAM}' param cannot be set with \
            {PREFIX_PARAM} and {SUFFIX_PARAM} params."
        ),
        (Some(_), Some(_), None) => panic!(
            "'{NAME_PARAM}' param cannot be set with \
            {PREFIX_PARAM} param."
        ),
        (Some(_), None, Some(_)) => panic!(
            "{NAME_PARAM} param cannot be set with \
            {SUFFIX_PARAM} param."
        ),
        (Some(name), None, None) => name,
    }
}

fn default_visibility_factory() -> Visibility {
    let span = Span2::call_site();
    let pub_token = Token![pub](span);
    Visibility::Public(pub_token)
}

fn remove_attributes(field: &mut Field) {
    field.attrs.retain(|attr| {
        let path = attr.path();
        !(path.is_ident(DISABLE_ATTRIBUTE) || path.is_ident(CONFIG_ATTRIBUTE))
    });
}
