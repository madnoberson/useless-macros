# Useless Setter Maker

<p align="left">
    <a href="https://crates.io/crates/useless-setter-maker">
        <img src="https://img.shields.io/crates/v/useless_setter_maker" alt="Crate version">
    </a>
    <a>
        <img src="https://img.shields.io/badge/license-Apache 2.0-green?logo=rust" alt="License">
    </a>
    <a>
        <img src="https://img.shields.io/badge/rust-1.85.0-green?logo=rust" alt="Rust version">
    </a>
    <a href="https://github.com/madnoberson/useless-macros/actions/workflows/lint-and-test.yaml" target="_blank">
        <img src="https://img.shields.io/github/actions/workflow/status/madnoberson/useless-macros/lint-and-test.yaml?logo=github" alt="Status of passing 'lint-and-test' workflow">
    </a>
</p>

A procedural macro crate for generating builder-style setter methods for Rust structs, enabling a fluent, chainable API for initialization with customizable method names and visibility.

## License

This project is licensed under the Apache 2.0 License. See the [LICENSE](LICENSE) file for details.

## Examples

### Basic Usage
```rust
use useless_setter_maker::make_setters;

#[make_setters]
#[derive(Debug, PartialEq, Default)]
struct Person {
    name: String,
    age: u8,
}

let person = Person::default()
    .with_name("Alice")
    .with_age(30 as u8);
    
assert_eq!(person.name, "Alice");
assert_eq!(person.age, 30);
```

### Customizing Setters
```rust
use useless_setter_maker::make_setters;

#[make_setters]
#[derive(Debug, PartialEq, Default)]
struct Config {
    #[configure_setter(prefix = "set", visibility = "pub")]
    host: String,
    
    #[configure_setter(suffix = "number", visibility = "pub(crate)")]
    port: u16,
    
    #[configure_setter(name = "enable_logging")]
    #[configure_setter(name = "install_logging")]
    logging: bool,
    
    #[disable_setters]
    internal: bool,

    updated_at: Option<String>,
}

let config = Config::default()
    .set_host("localhost")
    .with_number(8080 as u16)
    .enable_logging(true)
    .install_logging(true)
    .with_updated_at("today");
    
assert_eq!(config.host, "localhost");
assert_eq!(config.port, 8080);
assert_eq!(config.logging, true);
assert_eq!(config.internal, false);
assert_eq!(config.updated_at, Some(String::from("today")));
```
