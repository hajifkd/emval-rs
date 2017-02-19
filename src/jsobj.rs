extern crate emval_sys;

extern crate libc;

use emval_sys::*;

use std::ffi::CString;
use std::mem::{transmute, size_of};
use std::os::raw::c_void;
use std::sync::{Once, ONCE_INIT};
use std::ptr;

use libc::malloc;

static STR_ID: &'static str = "string\0";
static OBJ_ID: &'static str = "object\0";
static INT_ID: &'static str = "integer\0";
static DOUBLE_ID: &'static str = "double\0";

trait JSObj {
    fn id() -> *const c_void;
    fn to_js(&self) -> *const c_void;
}

impl JSObj for String {
    fn id() {
        static REGISTER: Once = ONCE_INIT;

        Once.call_once(|| {
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

    fn to_js(&self) -> *const c_void {
        let len = self.len();
        let str_byte = size_of::<char>() * len;
        unsafe {
            let s: *mut c_void = malloc(size_of::<usize>() + str_byte);
            ptr::write(transmute(s), len); 
            let c = transmute(transmute::<_, usize>(s) + size_of::<usize>());
            ptr::copy(self.as_ptr(), c, str_byte);

            s
        }
    }
}

impl JSObj for isize {
}
