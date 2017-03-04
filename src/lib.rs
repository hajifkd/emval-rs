extern crate emval_sys;

#[macro_use]
extern crate lazy_static;

pub mod jsserialize;
pub mod jsobj;

pub use jsobj::{Args, JSObj};
pub use jsserialize::JSSerialize;

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
