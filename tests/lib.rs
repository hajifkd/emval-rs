extern crate emval;

use emval::*;

#[test]
fn getobj() {
    let obj = JSObj::global("obj");
    assert_eq!(obj.call_prop::<isize>("returnOne", args!()), 1isize);
    assert_eq!(obj.call_prop::<f64>("returnOne", args!()), 1.0);
}

#[test]
fn add() {
    let obj = JSObj::global("obj");
    assert_eq!(obj.call_prop::<isize>("add", args!(1isize, 0isize)), 1isize);
}

