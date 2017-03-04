extern crate emval_sys;

#[macro_use]
extern crate lazy_static;

pub mod js_serializable;
pub mod jsobj;

pub use jsobj::{Args, JSObj};
pub use js_serializable::JSSerializable;

#[macro_export]
macro_rules! args {
    () => {{
        use emval;
        let types = Vec::new();
        let values = Vec::new();
        emval::jsobj::Args::new(types, values)
    }};

    ( $( $x:expr ),* ) => {{
        use emval;

        let mut types = Vec::new();
        let mut values = Vec::new();
        $(
            types.push($x.instance_id());
            values.push($x.serialize());
        )*

        emval::jsobj::Args::new(types, values)
    }};
}
