extern crate emval_sys;

extern crate libc;

use emval_sys::*;

use std;
use std::mem::{transmute, size_of};
use std::os::raw::c_void;
use std::sync::{Once, ONCE_INIT};
use std::ptr;

use self::libc::malloc;

pub static STR_ID: &'static str = "string\0";
pub static OBJ_ID: &'static str = "object\0";
pub static INT_ID: &'static str = "integer\0";
pub static DOUBLE_ID: &'static str = "double\0";

pub trait JSSerializable {
    fn id(&self) -> *const c_void;
    fn serialize(&self) -> EM_GENERIC_WIRE_TYPE;
}

pub fn to_wire_type(data: usize) -> EM_GENERIC_WIRE_TYPE {
    let mut result: EM_GENERIC_WIRE_TYPE = 0f64;
    unsafe {
        ptr::write(transmute::<_, *mut usize>(&mut result as *mut f64), data);
    }

    result
}

impl JSSerializable for str {
    fn id(&self) -> *const c_void {
        static REGISTER: Once = ONCE_INIT;

        REGISTER.call_once(|| {
            unsafe {
                _embind_register_std_wstring(transmute(STR_ID.as_ptr()),
                                             size_of::<char>(),
                                             transmute(STR_ID.as_ptr()));
            }
        });

        unsafe {
            transmute(STR_ID.as_ptr())
        }
    }

    fn serialize(&self) -> EM_GENERIC_WIRE_TYPE {
        let chars: Vec<char> = self.chars().collect();
        let len = chars.len();
        let bytes_len = len * size_of::<char>();
        unsafe {
            let s: *mut usize = malloc(size_of::<usize>() + bytes_len) as _;
            ptr::write(s, len); 
            let c = s.offset(1) as *mut u8;
            ptr::copy(chars.as_ptr() as _, c, bytes_len);

            to_wire_type(transmute(s))
        }
    }
}

impl JSSerializable for isize {
    fn id(&self) -> *const c_void {
        static REGISTER: Once = ONCE_INIT;

        REGISTER.call_once(|| {
            unsafe {
                _embind_register_integer(transmute(INT_ID.as_ptr()),
                                         transmute(INT_ID.as_ptr()),
                                         size_of::<isize>(),
                                         std::isize::MIN as _,
                                         std::isize::MAX as _);
            }
        });

        unsafe {
            transmute(INT_ID.as_ptr())
        }
    }

    fn serialize(&self) -> EM_GENERIC_WIRE_TYPE {
        unsafe {
            to_wire_type(transmute(*self))
        }
    }
}

impl JSSerializable for f64 {
    fn id(&self) -> *const c_void {
        static REGISTER: Once = ONCE_INIT;

        REGISTER.call_once(|| {
            unsafe {
                _embind_register_float(transmute(DOUBLE_ID.as_ptr()),
                                       transmute(DOUBLE_ID.as_ptr()),
                                       size_of::<f64>());
            }
        });

        unsafe {
            transmute(DOUBLE_ID.as_ptr())
        }
    }

    fn serialize(&self) -> EM_GENERIC_WIRE_TYPE {
        *self
    }
}
