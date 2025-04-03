use proc_macro::TokenStream;
use syn::{
    ItemStruct,
    parse_macro_input,
};

mod implementation;
use implementation::do_add_setters;

/// Automatically generates builder-style setter methods for a
/// struct's fields. Each generated method follows the naming
/// pattern `<prefix>_<suffix>`, where `prefix` defaults
/// to `with` and `suffix` defaults to the field name. The method
/// takes a single parameter and accepts any type that implements
/// `Into<T>`, where `T` is the type of the corresponding field.
/// This macro enhances struct initialization by enabling a fluent,
/// chainable API.
///
/// The macro is applied to a struct using the `#[add_setters]`
/// attribute and works with structs that have named fields.
/// The original struct remains unchanged, and the setters return
/// a new instance with the updated field value. Use the
/// `#[disable_setter]` attribute on fields to prevent the
/// generation of setter methods for those fields. Customize the
/// method names using `#[setter_prefix = "prefix"]` to change the
/// prefix and `#[setter_suffix = "suffix"]` to change a suffix.
///
/// # Examples
///
/// ```rust
/// use useless_add_setters::add_setters;
///
/// #[add_setters]
/// #[derive(Debug, PartialEq, Default)]
/// struct Foo {
///     bar: u16,
///
///     #[setter_prefix = "set"]
///     baz: String,
///
///     #[disable_setter]
///     foobar: bool,
///
///     #[setter_suffix = "fb"]
///     foobaz: bool,
///
///     #[setter_prefix = "set"]
///     #[setter_suffix = "bb"]
///     barbaz: String,
/// }
///
/// let foo = Foo::default()
///     .with_bar(100 as u16)
///     .set_baz("some_text")
///     .with_fb(true)
///     .set_bb("some_text");
///
/// // Setter for `foobar` is disabled and will not compile:
/// // foo = foo.with_foobar(true);
///
/// let expected_foo = Foo {
///     bar: 100,
///     baz: String::from("some_text"),
///     foobar: false,
///     foobaz: true,
///     barbaz: String::from("some_text"),
/// };
/// assert_eq!(foo, expected_foo);
/// ```
#[proc_macro_attribute]
pub fn add_setters(_: TokenStream, input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as ItemStruct);
    do_add_setters(input)
}
