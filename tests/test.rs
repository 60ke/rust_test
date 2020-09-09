use hello_macro_derive::HelloMacro;
use hello_macro::HelloMacro;


#[derive(HelloMacro)]
struct Pancakes;

#[test]
fn test_derive_a() {
    assert_eq!("Hello, Macro! My name is Pancakes1".to_string(), Pancakes::hello_macro().parse().unwrap());
}
