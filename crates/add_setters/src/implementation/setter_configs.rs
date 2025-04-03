use syn::{
    Expr,
    Field,
    Fields,
    Lit,
    Meta,
};

const DISABLE_ATTRIBUTE: &str = "disable_setter";
const PREFIX_ATTRIBUTE: &str = "setter_prefix";
const SUFFIX_ATTRIBUTE: &str = "setter_suffix";

const DEFAULT_PREFIX: &str = "with";

pub struct SetterConfig<'a> {
    field: &'a Field,
    prefix: String,
    suffix: String,
}

impl<'a> SetterConfig<'a> {
    pub fn field(&self) -> &Field {
        self.field
    }

    pub fn prefix(&self) -> &str {
        &self.prefix
    }

    pub fn suffix(&self) -> &str {
        &self.suffix
    }
}

pub fn make_setter_configs(fields: &mut Fields) -> Vec<SetterConfig> {
    let fields = match fields {
        Fields::Named(fields) => &mut fields.named,
        _ => panic!("`add_setters` only supports structs with named fields."),
    };
    let mut setter_configs: Vec<SetterConfig> = Vec::new();

    for field in fields {
        let is_disabled = field
            .attrs
            .iter()
            .any(|attr| attr.path().is_ident(DISABLE_ATTRIBUTE));
        if is_disabled {
            remove_attributes(field);
            continue;
        }

        let prefix = extract_prefix_if_exists(field)
            .unwrap_or(String::from(DEFAULT_PREFIX));
        let suffix = extract_suffix_if_exists(field)
            .unwrap_or(field.ident.as_ref().unwrap().to_string());

        remove_attributes(field);

        let setter_config = SetterConfig {
            field: field,
            prefix: prefix,
            suffix: suffix,
        };
        setter_configs.push(setter_config);
    }

    setter_configs
}

fn extract_prefix_if_exists(field: &Field) -> Option<String> {
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

fn extract_suffix_if_exists(field: &Field) -> Option<String> {
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
        "'{0}' attribute must have #[{0} = \"<prefix>\"] format.",
        SUFFIX_ATTRIBUTE,
    );
}

fn remove_attributes(field: &mut Field) {
    field.attrs.retain(|attr| {
        let path = attr.path();

        !(path.is_ident(DISABLE_ATTRIBUTE)
            || path.is_ident(PREFIX_ATTRIBUTE)
            || path.is_ident(SUFFIX_ATTRIBUTE))
    });
}
