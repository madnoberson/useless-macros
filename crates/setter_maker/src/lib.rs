#![doc = include_str!("../README.md")]
use proc_macro::TokenStream;
use syn::{
    ItemStruct,
    parse_macro_input,
};

mod implementation;
use implementation::do_make_setters;

/// Generates builder-style setter methods for struct fields, enabling
/// a fluent, chainable API for struct initialization. Each setter
/// follows the pattern `<prefix>_<suffix>` (default: `with_<field_name>`),
/// accepts any type implementing `Into<T>` (where `T` is the field type),
/// and returns a modified instance.
///
/// Apply this macro to a struct with named fields using `#[make_setters]`.
/// The original struct remains unchanged, and setters provide a
/// convenient way to set field values in a chainable manner.
///
/// Multiple `#[configure_setter]` attributes can be applied to a single field,
/// generating multiple setter methods with the specified configurations.
///
/// ### Customization Options
/// - `#[disable_setters]`: Skip setter generation for a specific field.
///
/// - `#[configure_setter(
///       name = "<name>",
///       visibility = "<vis>",
///       prefix = "<prefix>",
///       suffix = "<suffix>"
///    )]`:
///
///   Configure the setter with the following options:
///   - `name`: Set a custom method name, overriding prefix/suffix.
///   - `visibility`: Override method visibility. Set to "" for
///      `pub(self)`. Default: `pub`.
///   - `prefix`: Override the prefix. Default: "with".
///   - `suffix`: Override the suffix. Default: field name.
///
/// # Example
/// ```rust
/// use useless_setter_maker::make_setters;
///
/// #[make_setters]
/// #[derive(Debug, PartialEq, Default)]
/// struct Foo {
///     bar: u16,
///
///     #[configure_setter(prefix = "set", visibility = "pub")]
///     #[configure_setter(name = "install_baz", visibility = "pub(crate)")]
///     baz: String,
///
///     #[disable_setters]
///     foobar: bool,
///
///     #[configure_setter(suffix = "fb", visibility = "pub(crate)")]
///     foobaz: bool,
///
///     #[configure_setter(prefix = "provide", suffix = "bb")]
///     barbaz: String,
///
///     #[configure_setter(name = "install_bazfoo", visibility = "")]
///     bazfoo: String,
///
///     bazbaz: Option<u16>,
/// }
///
/// let foo = Foo::default()
///     .with_bar(100 as u16)       // Pub
///     .set_baz("some_text")       // Pub, first setter for baz
///     .install_baz("some_text")   // Pub(crate), second setter for baz
///     .with_fb(true)              // Pub(crate)
///     .provide_bb("other_text")   // Pub
///     .install_bazfoo("bazfoo")   // Pub(self)
///     .with_bazbaz(120 as u16);   // Pub   
///
/// let expected = Foo {
///     bar: 100,
///     baz: String::from("some_text"),
///     foobar: false,
///     foobaz: true,
///     barbaz: String::from("other_text"),
///     bazfoo: String::from("bazfoo"),
///     bazbaz: Some(120),
/// };
/// assert_eq!(foo, expected);
/// ```
#[proc_macro_attribute]
pub fn make_setters(_: TokenStream, input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as ItemStruct);
    do_make_setters(input)
}
