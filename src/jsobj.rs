extern crate emval_sys;


use emval_sys::*;

use std::ffi::CString;
use std::mem::transmute;
use std::sync::{Once, ONCE_INIT};

use js_serializable::*;

#[derive(Debug)]
pub struct Args {
    pub types: Vec<TYPEID>,
    pub values: Vec<EM_GENERIC_WIRE_TYPE>,
}

impl Args {
    pub fn new(types: Vec<TYPEID>, values: Vec<EM_GENERIC_WIRE_TYPE>) -> Args {
        Args { types: types, values: values}
    }

    pub fn len(&self) -> usize {
        self.types.len()
    }
}

#[derive(Debug)]
pub struct JSObj {
    val: EM_VAL,
}

impl JSSerializable for JSObj {
    type V = JSObj;

    fn id() -> TYPEID {
        static REGISTER: Once = ONCE_INIT;
        static OBJ_ID: &'static str = "rust_jsobj";

        REGISTER.call_once(|| {
            unsafe {
                _embind_register_emval(OBJ_ID.as_ptr() as _,
                                       OBJ_ID.as_ptr() as _);
            }
        });

        OBJ_ID.as_ptr() as _
    }

    fn serialize(&self) -> EM_GENERIC_WIRE_TYPE {
        unsafe {
            to_wire_type(transmute(self.val))
        }
    }

    fn deserialize(val: EM_GENERIC_WIRE_TYPE) -> JSObj {
        let val: *const EM_VAL = (&val as *const EM_GENERIC_WIRE_TYPE) as _;
        unsafe { JSObj { val: *val} }
    }
}

impl JSObj {
    pub fn global(name: &str) -> JSObj {
        let v = unsafe {
            _emval_get_global(CString::new(name).unwrap().as_ptr())
        };
        
        JSObj { val: v }
    }

    pub fn call(&self, args: Args) {
        unsafe {
            _emval_call(self.val, args.len() as _, args.types.as_ptr(),
                        args.values.as_ptr() as _);
        }
    }

    pub fn get_prop(&self, key: &str) -> JSObj {
        unsafe {
            JSObj { val: _emval_get_property(self.val,
                                             _emval_new_cstring(CString::new(key).unwrap().as_ptr())) }
        }
    }

    pub fn call_prop<T: JSSerializable>(&self, method_name: &str, args: Args) -> T::V {
        T::call_method(self.val, method_name, args)
    }
}

