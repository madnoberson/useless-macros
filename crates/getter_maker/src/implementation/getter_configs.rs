use core::panic;
use std::collections::HashMap;

use syn::{
    Attribute,
    Expr,
    Field,
    Fields,
    Ident,
    Lit,
    Meta,
    MetaNameValue,
    Token,
    Visibility,
    punctuated::Punctuated,
};

const CONFIG_ATTRIBUTE: &str = "configure_getter";
const DISABLE_ATTRIBUTE: &str = "disable_getters";

const NAME_PARAM: &str = "name";
const PREFIX_PARAM: &str = "prefix";
const SUFFIX_PARAM: &str = "suffix";
const VISIBILITY_PARAM: &str = "visibility";
const REF_STRATEGY_PARAM: &str = "ref_strategy";

const ALLOWED_REF_STRATEGIES: [&str; 2] = ["ref", "none"];

pub type GetterConfigs<'a> = HashMap<&'a Field, Vec<GetterConfig>>;

#[derive(Debug)]
pub struct GetterConfig {
    name: String,
    visibility: Visibility,
    ref_strategy: GetterRefStrategy,
}

#[derive(Debug, Clone, Copy)]
pub enum GetterRefStrategy {
    Ref,
    None,
}

impl TryFrom<String> for GetterRefStrategy {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.as_str() {
            "ref" => Ok(Self::Ref),
            "none" => Ok(Self::None),
            _ => Err(format!("Invalid visibility: '{value}'.")),
        }
    }
}

impl GetterConfig {
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn visibility(&self) -> &Visibility {
        &self.visibility
    }

    pub fn ref_strategy(&self) -> GetterRefStrategy {
        self.ref_strategy
    }
}

/// Extracts configs from config attributes of field and returns
/// them. If field has no attribute, this function returns Vec
/// with a default config.
pub fn make_getter_configs(fields: &mut Fields) -> GetterConfigs {
    let fields = match fields {
        Fields::Named(fields) => &mut fields.named,
        _ => panic!("Macro supports only structs with named fields."),
    };
    let mut getter_configs: GetterConfigs = Vec::new();

    for field in fields {
        let is_disabled = field
            .attrs
            .iter()
            .any(|attr| attr.path().is_ident(DISABLE_ATTRIBUTE));

        if is_disabled {
            remove_attributes(field);
            continue;
        }

        let name = extract_name(field).unwrap_or_else(|| {
            let prefix = extract_prefix(field);
            let suffix = extract_suffix(field);

            if prefix.is_some() && suffix.is_none() {
                return format!(
                    "{0}_{1}",
                    prefix.unwrap(),
                    field.ident.as_ref().unwrap()
                );
            } else if prefix.is_none() && suffix.is_some() {
                panic!(
                    "'{0}' attribute must be used with '{1}' attribute.",
                    SUFFIX_ATTRIBUTE, PREFIX_ATTRIBUTE,
                )
            } else if prefix.is_none() && suffix.is_none() {
                return field.ident.as_ref().unwrap().to_string();
            }

            format!("{0}_{1}", prefix.unwrap(), suffix.unwrap())
        });

        let visibility = extract_visibility(field)
            .map(|v| v.try_into().unwrap_or_else(|e| panic!("{}", e)))
            .unwrap_or(GetterVisibility::Pub);
        let ref_strategy = extract_ref_strategy(field)
            .map(|v| v.try_into().unwrap_or_else(|e| panic!("{}", e)))
            .unwrap_or(GetterRefStrategy::None);

        remove_attributes(field);

        let getter_config = GetterConfig {
            field,
            name,
            visibility,
            ref_strategy,
        };
        getter_configs.push(getter_config);
    }

    getter_configs
}

fn extact_config(field_ident: &Ident, attribute: &Attribute) -> GetterConfig {
    let name_values: Punctuated<MetaNameValue, Token![,]> = attribute
        .parse_args_with(Punctuated::parse_terminated)
        .unwrap();

    let mut name: Option<String> = None;
    let mut prefix: Option<String> = None;
    let mut suffix: Option<String> = None;
    let mut raw_visibility: Option<String> = None;
    let mut raw_ref_strategy: Option<String> = None;

    for name_value in name_values {
        if let Expr::Lit(lit_expr) = name_value.value {}
    }
}

fn remove_attributes(field: &mut Field) {
    field.attrs.retain(|attr| {
        let path = attr.path();
        !(path.is_ident(DISABLE_ATTRIBUTE) || path.is_ident(CONFIG_ATTRIBUTE))
    });
}
