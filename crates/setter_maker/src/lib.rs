#![doc = include_str!("../README.md")]
use proc_macro::TokenStream;
use syn::{
    ItemStruct,
    parse_macro_input,
};

mod builder_setters;
use builder_setters::do_make_builder_setters;

mod basic_setters;
use basic_setters::do_make_basic_setters;

/// Generates builder-style setter methods for struct fields, enabling
/// a fluent, chainable API for struct initialization. Each setter
/// follows the pattern `<prefix>_<suffix>` (default: `with_<field_name>`),
/// accepts any type implementing `Into<T>` (where `T` is the field type),
/// and returns a modified instance.
///
/// Apply this macro to a struct with named fields using
/// `#[make_builder_setters]`. The original struct remains unchanged,
/// and setters provide a convenient way to set field values in a
/// chainable manner.
///
/// Multiple `#[builder_setter]` attributes can be applied to a
/// single field, generating multiple setter methods with the specified
/// configurations.
///
/// ### Customization Options
/// - `#[disable_builder_setters]`: Skip setters generation for a
///    specific field.
///
/// - `#[builder_setter(
///       name = "<name>",
///       visibility = "<vis>",
///       prefix = "<prefix>",
///       suffix = "<suffix>",
///       with_into = true|false,
///    )]`:
///
///    Configure the setter with the following options:
///    - `name`: Set a custom method name, overriding prefix/suffix.
///    - `visibility`: Override method visibility. Set to "" for
///      `pub(self)`. Default: `pub`.
///    - `prefix`: Override the prefix. Default: "with".
///    - `suffix`: Override the suffix. Default: field name.
///    - `with_into`: Whether to use the `impl Into<T>` in method
///       parameters. Default: true.
///
/// # Example
/// ```rust
/// use useless_setter_maker::make_builder_setters;
///
/// #[make_builder_setters]
/// #[derive(Debug, PartialEq, Default)]
/// struct Foo {
///     bar: u16,
///
///     #[builder_setter(prefix = "set", visibility = "pub")]
///     #[builder_setter(name = "install_baz", visibility = "pub(crate)")]
///     baz: String,
///
///     #[disable_builder_setters]
///     foobar: bool,
///
///     #[builder_setter(suffix = "fb", visibility = "pub(crate)")]
///     foobaz: bool,
///
///     #[builder_setter(prefix = "provide", suffix = "bb")]
///     barbaz: String,
///
///     #[builder_setter(name = "install_bazfoo", visibility = "")]
///     bazfoo: String,
///
///     #[builder_setter(with_into = false)]
///     bazbaz: Option<String>,
/// }
///
/// let foo = Foo::default()
///     .with_bar(100 as u16)                     // Pub
///     .set_baz("some_text")                     // Pub, first baz setter
///     .install_baz("some_text")                 // Pub(crate), second baz setter
///     .with_fb(true)                            // Pub(crate)
///     .provide_bb("other_text")                 // Pub
///     .install_bazfoo("bazfoo")                 // Pub(self)
///     .with_bazbaz(String::from("some_text"));  // Pub   
///
/// let expected = Foo {
///     bar: 100,
///     baz: String::from("some_text"),
///     foobar: false,
///     foobaz: true,
///     barbaz: String::from("other_text"),
///     bazfoo: String::from("bazfoo"),
///     bazbaz: Some(String::from("some_text")),
/// };
///
/// assert_eq!(foo, expected);
/// ```
#[proc_macro_attribute]
pub fn make_builder_setters(_: TokenStream, item: TokenStream) -> TokenStream {
    let item = parse_macro_input!(item as ItemStruct);
    do_make_builder_setters(item)
}

/// Generates basic setter methods for struct fields. Each setter
/// follows the pattern `<prefix>_<suffix>` (default: `set_<field_name>`),
/// accepts any type implementing `Into<T>` (where `T` is the field type),
/// and returns a modified instance.
///
/// Apply this macro to a struct with named fields using
/// `#[make_basic_setters]`. The original struct remains unchanged,
/// and setters provide a convenient way to set field values..
///
/// Multiple `#[basic_setter]` attributes can be applied to a
/// single field, generating multiple setter methods with the specified
/// configurations.
///
/// ### Customization Options
/// - `#[disable_basic_setters]`: Skip setters generation for a
///    specific field.
///
/// - `#[basic_setter(
///       name = "<name>",
///       visibility = "<vis>",
///       prefix = "<prefix>",
///       suffix = "<suffix>",
///       with_into = true|false,
///    )]`:
///
///    Configure the setter with the following options:
///    - `name`: Set a custom method name, overriding prefix/suffix.
///    - `visibility`: Override method visibility. Set to "" for
///      `pub(self)`. Default: `pub`.
///    - `prefix`: Override the prefix. Default: "set".
///    - `suffix`: Override the suffix. Default: field name.
///    - `with_into`: Whether to use the `impl Into<T>` in method
///       parameters. Default: true.
///
/// # Example
/// ```rust
/// use useless_setter_maker::make_basic_setters;
///
/// #[make_basic_setters]
/// #[derive(Debug, PartialEq, Default)]
/// struct Foo {
///     bar: u16,
///
///     #[basic_setter(prefix = "with", visibility = "pub")]
///     #[basic_setter(name = "install_baz", visibility = "pub(crate)")]
///     baz: String,
///
///     #[disable_basic_setters]
///     foobar: bool,
///
///     #[basic_setter(suffix = "fb", visibility = "pub(crate)")]
///     foobaz: bool,
///
///     #[basic_setter(prefix = "provide", suffix = "bb")]
///     barbaz: String,
///
///     #[basic_setter(name = "install_bazfoo", visibility = "")]
///     bazfoo: String,
///
///     #[basic_setter(with_into = false)]
///     bazbaz: Option<String>,
/// }
///
/// let mut foo = Foo::default();
///
/// foo.set_bar(100 as u16);                    // Pub
/// foo.with_baz("some_text");                  // Pub, first baz setter
/// foo.install_baz("some_text");               // Pub(crate), second baz setter
/// foo.set_fb(true);                           // Pub(crate)
/// foo.provide_bb("other_text");               // Pub
/// foo.install_bazfoo("bazfoo");               // Pub(self)
/// foo.set_bazbaz(String::from("some_text"));  // Pub   
///
/// let expected = Foo {
///     bar: 100,
///     baz: String::from("some_text"),
///     foobar: false,
///     foobaz: true,
///     barbaz: String::from("other_text"),
///     bazfoo: String::from("bazfoo"),
///     bazbaz: Some(String::from("some_text")),
/// };
///
/// assert_eq!(foo, expected);
/// ```
#[proc_macro_attribute]
pub fn make_basic_setters(_: TokenStream, item: TokenStream) -> TokenStream {
    let item = parse_macro_input!(item as ItemStruct);
    do_make_basic_setters(item)
}
