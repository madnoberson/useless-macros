use core::panic;
use std::collections::HashMap;

use syn::{
    Expr,
    Field,
    Fields,
    Lit,
    LitStr,
    MetaNameValue,
    Path,
    Token,
    punctuated::Punctuated,
};

const CONFIG_ATTRIBUTE: &str = "configure_setter";
const DISABLE_ATTRIBUTE: &str = "disable_setters";

const NAME_PARAM: &str = "name";
const PREFIX_PARAM: &str = "prefix";
const SUFFIX_PARAM: &str = "suffix";
const VISIBILITY_PARAM: &str = "visibility";

const DEFAULT_PREFIX: &str = "with";

pub type SetterConfigs<'a> = HashMap<&'a Field, SetterConfig>;

#[derive(Debug)]
pub struct SetterConfig {
    name: String,
    visibility: SetterVisibility,
}

#[derive(Debug, Clone, Copy)]
pub enum SetterVisibility {
    Pub,
    PubForCrate,
    Private,
}

impl TryFrom<String> for SetterVisibility {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.as_str() {
            "pub" => Ok(Self::Pub),
            "pub(crate)" => Ok(Self::PubForCrate),
            "private" => Ok(Self::Private),
            _ => Err(format!("Invalid visibility: '{value}'.")),
        }
    }
}

impl SetterConfig {
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn visibility(&self) -> SetterVisibility {
        self.visibility
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

        let setter_config = extract_config(field);
        remove_attributes(field);
        setter_configs.insert(field, setter_config);
    }

    setter_configs
}

fn extract_config(field: &Field) -> SetterConfig {
    let attribute = field
        .attrs
        .iter()
        .filter(|attr| attr.path().is_ident(CONFIG_ATTRIBUTE))
        .last();

    if attribute.is_none() {
        let field_name = field.ident.as_ref().unwrap().to_string();
        let name = format!("{DEFAULT_PREFIX}_{field_name}");
        return SetterConfig {
            name,
            visibility: SetterVisibility::Pub,
        };
    }

    let name_values: Punctuated<MetaNameValue, Token![,]> = attribute
        .unwrap()
        .parse_args_with(Punctuated::parse_terminated)
        .expect("Inalid attribute syntax.");

    let mut name: Option<String> = None;
    let mut prefix: Option<String> = None;
    let mut suffix: Option<String> = None;
    let mut raw_visibility: Option<String> = None;

    for name_value in name_values {
        if let Expr::Lit(lit_expr) = name_value.value {
            if let Lit::Str(lit_str) = lit_expr.lit {
                parse_attribute_param(
                    name_value.path,
                    lit_str,
                    &mut name,
                    &mut prefix,
                    &mut suffix,
                    &mut raw_visibility,
                );
            }
        }
    }

    match (name.as_ref(), prefix.as_ref(), suffix.as_ref()) {
        (None, Some(prefix), Some(suffix)) => {
            let new_name = format!("{prefix}_{suffix}");
            name = Some(new_name);
        }
        (None, Some(prefix), None) => {
            let field_name = field.ident.as_ref().unwrap().to_string();
            let new_name = format!("{prefix}_{field_name}");
            name = Some(new_name);
        }
        (None, None, Some(suffix)) => {
            let new_name = format!("{DEFAULT_PREFIX}_{suffix}");
            name = Some(new_name);
        }
        (None, None, None) => {
            let field_name = field.ident.as_ref().unwrap().to_string();
            let new_name = format!("{DEFAULT_PREFIX}_{field_name}");
            name = Some(new_name);
        }
        (Some(_), _, _) => {}
    }

    let visibility = match raw_visibility {
        Some(raw_visibility) => {
            SetterVisibility::try_from(raw_visibility.clone())
                .expect("Invalid visibility.")
        }
        None => SetterVisibility::Pub,
    };

    SetterConfig {
        name: name.unwrap(),
        visibility,
    }
}

fn parse_attribute_param(
    param_path: Path,
    param_value: LitStr,
    name: &mut Option<String>,
    prefix: &mut Option<String>,
    suffix: &mut Option<String>,
    raw_visibility: &mut Option<String>,
) {
    let param_name = param_path
        .get_ident()
        .expect("Invalid attribute.")
        .to_string();

    match param_name.as_str() {
        NAME_PARAM => name.insert(param_value.value()),
        PREFIX_PARAM => prefix.insert(param_value.value()),
        SUFFIX_PARAM => suffix.insert(param_value.value()),
        VISIBILITY_PARAM => raw_visibility.insert(param_value.value()),
        _ => panic!("Invalid param of attribute."),
    };
}

fn remove_attributes(field: &mut Field) {
    field.attrs.retain(|attr| {
        let path = attr.path();
        !(path.is_ident(DISABLE_ATTRIBUTE) || path.is_ident(CONFIG_ATTRIBUTE))
    });
}
