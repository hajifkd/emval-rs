extern crate emval;

use emval::*;

macro_rules! test_for_types {
    ($obj: expr, $name: expr, $args: expr, $expect: expr, $( $t: ident )*) => {{
        $(
            assert_eq!($obj.call_prop::<$t>($name, $args), $expect);
        )*
    }}
}

#[test]
fn get_obj() {
    let obj = JSObj::global("obj");

    test_for_types!(obj, "returnOne", args!(), 1, isize i32 i16 i8 usize u32 u16 u8);
    test_for_types!(obj, "returnOne", args!(), 1.0, f64 f32);
}

#[test]
fn get_boolean() {
    let obj = JSObj::global("obj");

    assert_eq!(obj.call_prop::<bool>("returnTrue", args!()), true);
    assert_eq!(obj.call_prop::<bool>("returnFalse", args!()), false);
    assert_eq!(obj.call_prop::<bool>("returnNot", args!(true)), false);
    assert_eq!(obj.call_prop::<bool>("returnNot", args!(false)), true);
}

#[test]
fn add() {
    let obj = JSObj::global("obj");

    test_for_types!(obj, "add", args!(1, 2), 3, isize i32 i16 i8 usize u32 u16 u8);
    test_for_types!(obj, "add", args!(1.0, 2.0), 3.0, f64 f32);
}

#[test]
fn get_string() {
    let obj = JSObj::global("obj");
    assert_eq!(obj.call_prop::<String>("helloWorld", args!()), "Hello, world!");
}

#[test]
fn get_wstring() {
    let obj = JSObj::global("obj");
    assert_eq!(obj.call_prop::<String>("helloWorldMulti", args!()), "こんにちは、世界!");
}

#[test]
fn get_surrogate_pair() {
    let obj = JSObj::global("obj");
    //assert_eq!(obj.call_prop::<String>("helloSurrogatePair", args!()), "🍺");
    // This is the bug in emscripten itself!
}
