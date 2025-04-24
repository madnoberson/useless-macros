use std::{
    ascii,
    fmt::Debug,
};

use useless_getter_maker::make_getters;

#[test]
fn test_basic_scenario() {
    #[make_getters]
    #[derive(Debug, PartialEq, Default)]
    struct Foo {
        bar: u16,
        #[getter_ref_strategy = "ref"]
        baz: String,
    }

    let foo = Foo {
        bar: 12,
        baz: String::from("asd"),
    };

    assert_eq!(foo.bar(), 12);
    assert_eq!(foo.baz(), &String::from("asd"));
}

#[test]
fn test_basic_scenario_with_generics() {
    #[make_getters]
    #[derive(Debug, PartialEq, Default)]
    pub struct Foo<T>
    where
        T: Debug + PartialEq + Default,
    {
        bar: u16,
        #[getter_ref_strategy = "ref"]
        baz: T,
    }

    let foo = Foo {
        bar: 12,
        baz: vec![1, 2, 3],
    };

    assert_eq!(foo.bar(), 12);
    assert_eq!(foo.baz(), &vec![1, 2, 3]);
}
