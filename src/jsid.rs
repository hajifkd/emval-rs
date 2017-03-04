extern crate emval_sys;

use emval_sys::*;

use std;
use std::sync::{Once, ONCE_INIT};
use std::mem::size_of;

pub trait JSID {
    fn id() -> TYPEID;

    // For macro...
    fn instance_id(&self) -> TYPEID {
        Self::id()
    }
}

impl JSID for () {
    fn id() -> TYPEID {
        static REGISTER: Once = ONCE_INIT;
        static VOID_ID: &'static str = "rust_void";

        REGISTER.call_once(|| {
            unsafe {
                _embind_register_void(VOID_ID.as_ptr() as _,
                                      VOID_ID.as_ptr() as _);
            }
        });

        VOID_ID.as_ptr() as _
    }
}

impl JSID for str {
    fn id() -> TYPEID {
        static REGISTER: Once = ONCE_INIT;
        static STR_ID: &'static str = "rust_string_utf32";

        REGISTER.call_once(|| {
            unsafe {
                _embind_register_std_wstring(STR_ID.as_ptr() as _,
                                             size_of::<char>(),
                                             STR_ID.as_ptr() as _);
            }
        });

        STR_ID.as_ptr() as _
    }
}

impl JSID for String {
    fn id() -> TYPEID {
        str::id()
    }
}

impl JSID for bool {
    fn id() -> TYPEID {
        static REGISTER: Once = ONCE_INIT;
        static BOOL_ID: &'static str = "rust_bool";

        REGISTER.call_once(|| {
            unsafe {
                _embind_register_bool(BOOL_ID.as_ptr() as _,
                                      BOOL_ID.as_ptr() as _,
                                      size_of::<bool>(),
                                      true,
                                      false);
            }
        });

        BOOL_ID.as_ptr() as _
    }
}

macro_rules! register_rust_integer {
    ( $( $t:ident )* ) => {
        $(
            impl JSID for $t {
                fn id() -> TYPEID {
                    static REGISTER: Once = ONCE_INIT;
                    static INT_ID: &'static str = concat!("rust_", stringify!($t));

                    REGISTER.call_once(|| {
                        unsafe {
                            _embind_register_integer(INT_ID.as_ptr() as _,
                                                     INT_ID.as_ptr() as _,
                                                     size_of::<$t>(),
                                                     std::$t::MIN as _,
                                                     std::$t::MAX as _);
                        }
                    });

                    INT_ID.as_ptr() as _
                }
            }
        )*
    }
}

register_rust_integer!(isize i32 i16 i8 usize u32 u16 u8);

macro_rules! register_rust_real {
    ( $( $t:ident )* ) => {
        $(
            impl JSID for $t {
                fn id() -> TYPEID {
                    static REGISTER: Once = ONCE_INIT;
                    static DOUBLE_ID: &'static str = concat!("rust_", stringify!($t));

                    REGISTER.call_once(|| {
                        unsafe {
                            _embind_register_float(DOUBLE_ID.as_ptr() as _,
                                                   DOUBLE_ID.as_ptr() as _,
                                                   size_of::<$t>());
                        }
                    });

                    DOUBLE_ID.as_ptr() as _
                }
            }
        )*
    }
}

register_rust_real!(f64 f32);
