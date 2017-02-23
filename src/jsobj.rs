extern crate emval_sys;


use emval_sys::*;

use std::ffi::CString;
use std::mem::transmute;
use std::os::raw::c_void;
use std::sync::{Once, ONCE_INIT};

use js_serializable::*;

lazy_static! {
    static ref VOID_ID: &'static str = unsafe {
        let name = "void\0";
        _embind_register_void(name.as_ptr() as _,
                              name.as_ptr() as _);
        name
    };
}

#[derive(Debug)]
pub struct Args {
    types: Vec<TYPEID>,
    values: Vec<EM_GENERIC_WIRE_TYPE>,
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
    fn id(&self) -> *const c_void {
        static REGISTER: Once = ONCE_INIT;

        REGISTER.call_once(|| {
            unsafe {
                _embind_register_emval(transmute(OBJ_ID.as_ptr()),
                                       transmute(OBJ_ID.as_ptr()));
            }
        });

        unsafe {
            transmute(OBJ_ID.as_ptr())
        }
    }

    fn serialize(&self) -> EM_GENERIC_WIRE_TYPE {
        unsafe {
            to_wire_type(transmute(self.val))
        }
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

    pub fn call_void_prop(&self, key: &str, args: Args) {
        unsafe {
            let mut types: Vec<TYPEID> = vec![transmute(0usize); args.len() + 1];
            types[0] = VOID_ID.as_ptr() as _;
            types[1..].clone_from_slice(&args.types);

            let caller = _emval_get_method_caller(types.len() as _,
                                                  types.as_ptr() as _);
            _emval_call_void_method(caller, self.val,
                                    CString::new(key).unwrap().as_ptr(),
                                    args.values.as_ptr() as _);
        }
    }
}

