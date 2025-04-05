use std::fmt::Debug;

use useless_add_setters::add_setters;

#[test]
fn test_basic_scenario() {
    #[add_setters]
    #[derive(Debug, PartialEq, Default)]
    struct Foo {
        bar: u16,
        baz: String,
    }

    let foo = Foo::default().with_bar(100 as u16).with_baz("some_text");
    let expected_foo = Foo {
        bar: 100 as u16,
        baz: String::from("some_text"),
    };

    assert_eq!(foo, expected_foo)
}

#[test]
fn test_basic_scenario_with_generics() {
    #[add_setters]
    #[derive(Debug, PartialEq, Default)]
    pub struct Foo<T>
    where
        T: Debug + PartialEq + Default,
    {
        bar: u16,
        baz: T,
    }

    let foo = Foo::<String>::default()
        .with_bar(100 as u16)
        .with_baz("some_text");
    let expected_foo = Foo {
        bar: 100 as u16,
        baz: String::from("some_text"),
    };

    assert_eq!(foo, expected_foo)
}
