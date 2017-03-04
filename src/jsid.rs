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

impl JSID for isize {
    fn id() -> TYPEID {
        static REGISTER: Once = ONCE_INIT;
        static INT_ID: &'static str = "rust_integer";

        REGISTER.call_once(|| {
            unsafe {
                _embind_register_integer(INT_ID.as_ptr() as _,
                                         INT_ID.as_ptr() as _,
                                         size_of::<isize>(),
                                         std::isize::MIN as _,
                                         std::isize::MAX as _);
            }
        });

        INT_ID.as_ptr() as _
    }
}

impl JSID for f64 {
    fn id() -> TYPEID {
        static REGISTER: Once = ONCE_INIT;
        static DOUBLE_ID: &'static str = "rust_double";

        REGISTER.call_once(|| {
            unsafe {
                _embind_register_float(DOUBLE_ID.as_ptr() as _,
                                       DOUBLE_ID.as_ptr() as _,
                                       size_of::<f64>());
            }
        });

        DOUBLE_ID.as_ptr() as _
    }
}

