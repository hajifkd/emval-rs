extern crate emval_sys;

pub mod js_serializable;
pub mod jsobj;

#[macro_export]
macro_rules! args {
    ( $( $x:expr ),* ) => {{
        let mut types = Vec::new();
        let mut values = Vec::new();
        $(
            types.push($x.id());
            values.push($x.serialize());
        )*

        emval::jsobj::Args::new(types, values)
    }};
}
