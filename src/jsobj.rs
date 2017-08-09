extern crate emval_sys;


use emval_sys::*;

use std::ffi::CString;
use std::mem::transmute;
use std::sync::{Once, ONCE_INIT};

use jsid::*;
use jsserialize::*;
use jsdeserialize::*;

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

#[derive(Debug, PartialEq)]
pub struct JSObj {
    val: EM_VAL,
}

impl Drop for JSObj {
    fn drop(&mut self) {
        unsafe {
            _emval_decref(self.val);
        }
    }
}

impl JSID for JSObj {
    fn id() -> TYPEID {
        static REGISTER: Once = ONCE_INIT;
        static OBJ_ID: &'static str = "rust_jsobj\0";

        REGISTER.call_once(|| {
            unsafe {
                _embind_register_emval(OBJ_ID.as_ptr() as _,
                                       OBJ_ID.as_ptr() as _);
            }
        });

        OBJ_ID.as_ptr() as _
    }
}

impl JSSerialize for JSObj {
    fn serialize(&self) -> EM_GENERIC_WIRE_TYPE {
        unsafe {
            _emval_incref(self.val);
            to_wire_type(transmute(self.val))
        }
    }
}

impl JSDeserialize for JSObj {
    fn from_jsobj_id(obj_id: EM_VAL) -> JSObj {
        JSObj { val: obj_id }
    }

    fn deserialize(val: EM_GENERIC_WIRE_TYPE) -> JSObj {
        JSObj { val: (val as usize) as _ }
    }
}

impl JSObj {
    pub fn undefined() -> JSObj {
        JSObj { val: _EMVAL_UNDEFINED }
    }

    pub fn null() -> JSObj {
        JSObj { val: _EMVAL_NULL }
    }

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

    pub fn call_prop<T: JSDeserialize>(&self, method_name: &str, args: Args) -> T {
        T::call_method(self.val, method_name, args)
    }
}

