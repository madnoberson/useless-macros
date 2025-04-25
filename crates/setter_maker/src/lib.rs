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
/// ### Customization Options
/// - `#[disable_setters]`: Skip setter generation for a specific field.
///
/// - `#[configure_setter(
///       name = "name",
///       visibility = "<vis>",
///       prefix = "prefix",
///       suffix = "suffix"
///    )]`:
///   Configure the setter with the following options:
///   - `name`: Set a custom method name, overriding prefix/suffix.
///   - `visibility`: Control method visibility:
///     - `pub`: Public access (default).
///     - `pub(crate)`: Crate-only access.
///     - `private`: Private access.
///   - `prefix`: Override the default `with` prefix.
///   - `suffix`: Customize the suffix (defaults to field name).
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
///     #[configure_setter(name = "install_bazfoo", visibility = "private")]
///     bazfoo: String,
/// }
///
/// let foo = Foo::default()
///     .with_bar(100 as u16)       // Public
///     .set_baz("some_text")       // Public
///     .with_fb(true)              // Public(crate)
///     .provide_bb("other_text")   // Public
///     .install_bazfoo("bazfoo");  // Private
///
/// let expected = Foo {
///     bar: 100,
///     baz: String::from("some_text"),
///     foobar: false,
///     foobaz: true,
///     barbaz: String::from("other_text"),
///     bazfoo: String::from("bazfoo"),
/// };
/// assert_eq!(foo, expected);
/// ```
#[proc_macro_attribute]
pub fn make_setters(_: TokenStream, input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as ItemStruct);
    do_make_setters(input)
}
