use core::panic;

use syn::{
    Expr,
    Field,
    Fields,
    Lit,
    Meta,
};

const DISABLE_ATTRIBUTE: &str = "disable_getter";
const NAME_ATTRIBUTE: &str = "getter_name";
const PREFIX_ATTRIBUTE: &str = "getter_prefix";
const SUFFIX_ATTRIBUTE: &str = "getter_suffix";
const VISIBILITY_ATTRIBUTE: &str = "getter_visibility";
const REF_STRATEGY_ATTRIBUTE: &str = "getter_ref_strategy";

const ALLOWED_VISIBILITIES: [&str; 3] = ["pub", "pub(crate)", "private"];
const ALLOWED_REF_STRATEGIES: [&str; 2] = ["ref", "none"];

pub struct GetterConfig<'a> {
    field: &'a Field,
    name: String,
    visibility: GetterVisibility,
    ref_strategy: GetterRefStrategy,
}

#[derive(Clone, Copy)]
pub enum GetterVisibility {
    Pub,
    PubForCrate,
    Private,
}

impl TryFrom<String> for GetterVisibility {
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

#[derive(Clone, Copy)]
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

impl<'a> GetterConfig<'a> {
    pub fn field(&self) -> &Field {
        self.field
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn visibility(&self) -> GetterVisibility {
        self.visibility
    }

    pub fn ref_strategy(&self) -> GetterRefStrategy {
        self.ref_strategy
    }
}

pub fn make_getter_configs(fields: &mut Fields) -> Vec<GetterConfig> {
    let fields = match fields {
        Fields::Named(fields) => &mut fields.named,
        _ => panic!("`add_setters` only supports structs with named fields."),
    };
    let mut getter_configs: Vec<GetterConfig> = Vec::new();

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

fn extract_name(field: &Field) -> Option<String> {
    let attribute = field
        .attrs
        .iter()
        .filter(|attr| attr.path().is_ident(NAME_ATTRIBUTE))
        .last()?;

    if let Meta::NameValue(meta) = &attribute.meta {
        if let Expr::Lit(expr) = &meta.value {
            if let Lit::Str(lit_str) = &expr.lit {
                return Some(lit_str.value());
            }
        }
    }

    panic!(
        "'{0}' attribute must have #[{0} = \"<name>\"] format.",
        NAME_ATTRIBUTE,
    );
}

fn extract_prefix(field: &Field) -> Option<String> {
    let attribute = field
        .attrs
        .iter()
        .filter(|attr| attr.path().is_ident(PREFIX_ATTRIBUTE))
        .last()?;

    if let Meta::NameValue(meta) = &attribute.meta {
        if let Expr::Lit(expr) = &meta.value {
            if let Lit::Str(lit_str) = &expr.lit {
                return Some(lit_str.value());
            }
        }
    }

    panic!(
        "'{0}' attribute must have #[{0} = \"<prefix>\"] format.",
        PREFIX_ATTRIBUTE,
    );
}

fn extract_suffix(field: &Field) -> Option<String> {
    let attribute = field
        .attrs
        .iter()
        .filter(|attr| attr.path().is_ident(SUFFIX_ATTRIBUTE))
        .last()?;

    if let Meta::NameValue(meta) = &attribute.meta {
        if let Expr::Lit(expr) = &meta.value {
            if let Lit::Str(lit_str) = &expr.lit {
                return Some(lit_str.value());
            }
        }
    }

    panic!(
        "'{0}' attribute must have #[{0} = \"<suffix>\"] format.",
        SUFFIX_ATTRIBUTE,
    );
}

fn extract_visibility(field: &Field) -> Option<String> {
    let attribute = field
        .attrs
        .iter()
        .filter(|attr| attr.path().is_ident(VISIBILITY_ATTRIBUTE))
        .last()?;

    if let Meta::NameValue(meta) = &attribute.meta {
        if let Expr::Lit(expr) = &meta.value {
            if let Lit::Str(lit_str) = &expr.lit {
                return Some(lit_str.value());
            }
        }
    }

    panic!(
        "'{0}' attribute must have #[{0} = \"<visibility>\"] format, \
        where <visibility> must be one of ({1}).",
        VISIBILITY_ATTRIBUTE,
        ALLOWED_VISIBILITIES.join(", "),
    );
}

fn extract_ref_strategy(field: &Field) -> Option<String> {
    let attribute = field
        .attrs
        .iter()
        .filter(|attr| attr.path().is_ident(REF_STRATEGY_ATTRIBUTE))
        .last()?;

    if let Meta::NameValue(meta) = &attribute.meta {
        if let Expr::Lit(expr) = &meta.value {
            if let Lit::Str(lit_str) = &expr.lit {
                return Some(lit_str.value());
            }
        }
    }

    panic!(
        "'{0}' attribute must have #[{0} = \"<referencing strategy>\"] \"
        format, where <referencing strategy> must be one of ({1}).",
        REF_STRATEGY_ATTRIBUTE,
        ALLOWED_REF_STRATEGIES.join(", "),
    );
}

fn remove_attributes(field: &mut Field) {
    field.attrs.retain(|attr| {
        let path = attr.path();

        !(path.is_ident(DISABLE_ATTRIBUTE)
            || path.is_ident(NAME_ATTRIBUTE)
            || path.is_ident(PREFIX_ATTRIBUTE)
            || path.is_ident(SUFFIX_ATTRIBUTE)
            || path.is_ident(VISIBILITY_ATTRIBUTE)
            || path.is_ident(REF_STRATEGY_ATTRIBUTE))
    });
}
