extern crate emval_sys;

extern crate libc;

use emval_sys::*;

use std;
use std::ffi::CString;
use std::mem::{transmute, size_of};
use std::sync::{Once, ONCE_INIT};
use std::ptr;

use self::libc::malloc;

use jsobj::Args;

pub static STR_ID: &'static str = "rust_string\0";
pub static OBJ_ID: &'static str = "rust_js_object\0";
pub static INT_ID: &'static str = "rust_integer\0";
pub static DOUBLE_ID: &'static str = "rust_double\0";
pub static VOID_ID: &'static str = "rust_void\0";

pub trait JSSerializable {
    type V;

    fn id() -> TYPEID;

    // For macro...
    fn instance_id(&self) -> TYPEID {
        Self::id()
    }

    fn serialize(&self) -> EM_GENERIC_WIRE_TYPE;

    fn deserialize(v: EM_GENERIC_WIRE_TYPE) -> Self::V;

    fn call_method(handle: EM_VAL,
                   method_name: &str,
                   args: Args) -> Self::V {
        unsafe {
            let mut types: Vec<TYPEID> = vec![transmute(0usize); args.len() + 1];
            types[0] = Self::id();
            types[1..].clone_from_slice(&args.types);
            let caller = _emval_get_method_caller(types.len() as _,
                                                  types.as_ptr() as _);
            let mut destructors = transmute(0usize);
            let result = _emval_call_method(caller, handle,
                                            CString::new(method_name).unwrap().as_ptr(),
                                            &mut destructors as _,
                                            args.values.as_ptr() as _);
            _emval_run_destructors(destructors);

            Self::deserialize(result)
        }
    }
}

pub fn to_wire_type(data: usize) -> EM_GENERIC_WIRE_TYPE {
    let mut result: EM_GENERIC_WIRE_TYPE = 0f64;
    unsafe {
        ptr::write((&mut result as *mut f64) as _, data);
    }

    result
}

pub fn to_ptr(data: EM_GENERIC_WIRE_TYPE) -> isize {
    let ptr: *const isize = (&data as *const EM_GENERIC_WIRE_TYPE) as _;
    unsafe { *ptr }
}

impl JSSerializable for () {
    type V = ();

    fn id() -> TYPEID {
        static REGISTER: Once = ONCE_INIT;

        REGISTER.call_once(|| {
            unsafe {
                _embind_register_void(VOID_ID.as_ptr() as _,
                                      VOID_ID.as_ptr() as _);
            }
        });

        VOID_ID.as_ptr() as _
    }

    fn serialize(&self) -> EM_GENERIC_WIRE_TYPE {
        0f64
    }

    fn deserialize(_: EM_GENERIC_WIRE_TYPE) {
    }

    fn call_method(handle: EM_VAL,
                   method_name: &str,
                   args: Args) {
        unsafe {
            let mut types: Vec<TYPEID> = vec![transmute(0usize); args.len() + 1];
            types[0] = ().instance_id();
            types[1..].clone_from_slice(&args.types);

            let caller = _emval_get_method_caller(types.len() as _,
                                                  types.as_ptr() as _);
            _emval_call_void_method(caller, handle,
                                    CString::new(method_name).unwrap().as_ptr(),
                                    args.values.as_ptr() as _);
        }
    }
}

impl JSSerializable for str {
    type V = String;

    fn id() -> TYPEID {
        static REGISTER: Once = ONCE_INIT;

        REGISTER.call_once(|| {
            unsafe {
                _embind_register_std_wstring(STR_ID.as_ptr() as _,
                                             size_of::<char>(),
                                             STR_ID.as_ptr() as _);
            }
        });

        STR_ID.as_ptr() as _
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

    fn deserialize(ptr: EM_GENERIC_WIRE_TYPE) -> String {
        let ptr: *mut usize = unsafe { transmute(to_ptr(ptr)) };
        let len: usize = unsafe { *ptr };
        let bytes_len = len * size_of::<char>();
        let mut vec: Vec<u8> = Vec::with_capacity(bytes_len);

        unsafe {
            ptr::copy(ptr.offset(1) as _, vec.as_mut_ptr(), bytes_len);
        }

        String::from_utf8(vec).unwrap()
    }
}

impl JSSerializable for isize {
    type V = isize;

    fn id() -> TYPEID {
        static REGISTER: Once = ONCE_INIT;

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

    fn serialize(&self) -> EM_GENERIC_WIRE_TYPE {
        to_wire_type(*self as _)
    }

    fn deserialize(val: EM_GENERIC_WIRE_TYPE) -> isize {
        val as _
    }
}

impl JSSerializable for f64 {
    type V = f64;

    fn id() -> TYPEID {
        static REGISTER: Once = ONCE_INIT;

        REGISTER.call_once(|| {
            unsafe {
                _embind_register_float(DOUBLE_ID.as_ptr() as _,
                                       DOUBLE_ID.as_ptr() as _,
                                       size_of::<f64>());
            }
        });

        DOUBLE_ID.as_ptr() as _
    }

    fn serialize(&self) -> EM_GENERIC_WIRE_TYPE {
        *self
    }

    fn deserialize(val: EM_GENERIC_WIRE_TYPE) -> f64 {
        val
    }
}
