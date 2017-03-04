extern crate emval_sys;

extern crate libc;

use emval_sys::*;

use std::mem::{transmute, size_of};
use std::ptr;

use self::libc::malloc;

use jsid::*;

pub trait JSSerialize: JSID {
    fn serialize(&self) -> EM_GENERIC_WIRE_TYPE;

}

pub fn to_wire_type(data: usize) -> EM_GENERIC_WIRE_TYPE {
    let mut result: EM_GENERIC_WIRE_TYPE = 0f64;
    unsafe {
        ptr::write((&mut result as *mut f64) as _, data);
    }

    result
}

impl JSSerialize for () {
    fn serialize(&self) -> EM_GENERIC_WIRE_TYPE {
        0f64
    }
}

impl JSSerialize for str {
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

impl JSSerialize for isize {
    fn serialize(&self) -> EM_GENERIC_WIRE_TYPE {
        to_wire_type(*self as _)
    }
}

impl JSSerialize for f64 {
    fn serialize(&self) -> EM_GENERIC_WIRE_TYPE {
        *self
    }
}
