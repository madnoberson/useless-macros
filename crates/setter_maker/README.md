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

A procedural macro crate for generating setter methods for Rust structs.

## License

This project is licensed under the Apache 2.0 License. See the [LICENSE](LICENSE) file for details.

## Examples

### Builder setters

```rust
use useless_setter_maker::make_builder_setters;

#[make_builder_setters]
#[derive(Debug, PartialEq, Default)]
struct Config {
    #[builder_setter(prefix = "set", visibility = "pub")]
    host: String,
    
    #[builder_setter(suffix = "number", visibility = "pub(crate)")]
    port: u16,
    
    #[builder_setter(name = "enable_logging")]
    #[builder_setter(name = "install_logging")]
    logging: bool,
    
    #[disable_builder_setters]
    internal: bool,

    #[builder_setter(with_into = false)]
    updated_at: Option<String>,
}

let config = Config::default()
    .set_host("localhost")                    // Pub
    .with_number(8080 as u16)                 // Pub(crate)
    .enable_logging(true)                     // Pub, first logging setter  
    .install_logging(true)                    // Pub, second logging setter
    .with_updated_at(String::from("today"));  // Pub
    
assert_eq!(config.host, "localhost");
assert_eq!(config.port, 8080);
assert_eq!(config.logging, true);
assert_eq!(config.internal, false);
assert_eq!(config.updated_at, Some(String::from("today")));
```

### Basic setters

```rust
use useless_setter_maker::make_basic_setters;

#[make_basic_setters]
#[derive(Debug, PartialEq, Default)]
struct Config {
    #[basic_setter(prefix = "with", visibility = "pub")]
    host: String,
    
    #[basic_setter(suffix = "number", visibility = "pub(crate)")]
    port: u16,
    
    #[basic_setter(name = "enable_logging")]
    #[basic_setter(name = "install_logging")]
    logging: bool,
    
    #[disable_basic_setters]
    internal: bool,

    #[basic_setter(with_into = false)]
    updated_at: Option<String>,
}

let mut config = Config::default();

config.with_host("localhost");                  // Pub
config.set_number(8080 as u16);                 // Pub(crate)
config.enable_logging(true);                    // Pub, first logging setter  
config.install_logging(true);                   // Pub, second logging setter
config.set_updated_at(String::from("today"));   // Pub
    
assert_eq!(config.host, "localhost");
assert_eq!(config.port, 8080);
assert_eq!(config.logging, true);
assert_eq!(config.internal, false);
assert_eq!(config.updated_at, Some(String::from("today")));
```
