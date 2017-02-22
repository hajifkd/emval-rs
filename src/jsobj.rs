extern crate emval_sys;

extern crate libc;

use emval_sys::*;

use std::ffi::CString;
use std::mem::transmute;
use std::os::raw::c_void;
use std::sync::{Once, ONCE_INIT};

use js_serializable::*;

#[derive(Debug)]
pub struct JSObj {
    val: EM_VAL,
}

impl JSSerializable for JSObj {
    fn id() -> *const c_void {
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
}
