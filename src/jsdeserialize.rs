extern crate emval_sys;

use emval_sys::*;

use std;
use std::ffi::CString;
use std::mem::{transmute, size_of};
use std::ptr;

use jsid::*;
use jsobj::Args;

pub trait JSDeserialize: JSID {
    fn deserialize(v: EM_GENERIC_WIRE_TYPE) -> Self;

    fn call_method(handle: EM_VAL,
                   method_name: &str,
                   args: Args) -> Self 
        where Self: std::marker::Sized {
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
            let result = Self::deserialize(result);
            _emval_run_destructors(destructors);

            result
        }
    }
}

impl JSDeserialize for () {
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

impl JSDeserialize for String {
    fn deserialize(ptr: EM_GENERIC_WIRE_TYPE) -> String {
        let ptr: *mut usize = unsafe { transmute(ptr as usize) };
        let len: usize = unsafe { *ptr };
        let mut vec: Vec<char> = vec!['\0'; len];

        unsafe {
            ptr::copy(ptr.offset(1) as _, vec.as_mut_ptr(), len);
        }

        vec.into_iter().collect()
    }
}

impl JSDeserialize for isize {
    fn deserialize(val: EM_GENERIC_WIRE_TYPE) -> isize {
        val as _
    }
}

impl JSDeserialize for f64 {
    fn deserialize(val: EM_GENERIC_WIRE_TYPE) -> f64 {
        val
    }
}
