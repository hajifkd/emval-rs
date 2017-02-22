extern crate emval_sys;

pub mod js_serializable;
pub mod jsobj;

pub use jsobj::{Args, JSObj};
pub use js_serializable::JSSerializable;

#[macro_export]
macro_rules! args {
    ( $( $x:expr ),* ) => {{
        use emval;

        let mut types = Vec::new();
        let mut values = Vec::new();
        $(
            types.push($x.id());
            values.push($x.serialize());
        )*

        emval::jsobj::Args::new(types, values)
    }};
}
