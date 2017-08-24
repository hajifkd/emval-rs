extern crate emval_sys;

extern crate libc;

use emval_sys::*;

use std::mem::size_of;
use std::ptr;

use self::libc::malloc;

use jsid::*;

pub trait JSSerialize: JSID {
    type WireType;

    fn to_wire_type(&self) -> Self::WireType;
    fn serialize(&self) -> EM_GENERIC_WIRE_TYPE;
}

pub fn to_generic_wire_type(data: usize) -> EM_GENERIC_WIRE_TYPE {
    let mut result: EM_GENERIC_WIRE_TYPE = 0f64;
    unsafe {
        ptr::write((&mut result as *mut f64) as _, data);
    }

    result
}

impl JSSerialize for () {
    type WireType = usize;

    fn to_wire_type(&self) -> usize {
        0usize
    }

    fn serialize(&self) -> EM_GENERIC_WIRE_TYPE {
        to_generic_wire_type(self.to_wire_type())
    }
}

impl JSSerialize for str {
    type WireType = usize;

    fn to_wire_type(&self) -> usize {
        let chars: Vec<char> = self.chars().collect();
        let len = chars.len();
        let bytes_len = len * size_of::<char>();
        unsafe {
            let s: *mut usize = malloc(size_of::<usize>() + bytes_len) as _;
            ptr::write(s, len); 
            let c = s.offset(1) as *mut u8;
            ptr::copy(chars.as_ptr() as _, c, bytes_len);

            s as _
        }
    }

    fn serialize(&self) -> EM_GENERIC_WIRE_TYPE {
        to_generic_wire_type(self.to_wire_type())
    }

}

macro_rules! serialize_rust_integer {
    ( $( $t:ident )* ) => {
        $(
            impl JSSerialize for $t {
                type WireType = usize;

                fn to_wire_type(&self) -> usize {
                    *self as _
                }

                fn serialize(&self) -> EM_GENERIC_WIRE_TYPE {
                    to_generic_wire_type(self.to_wire_type())
                }
            }
        )*
    }
}

serialize_rust_integer!(isize i32 i16 i8 usize u32 u16 u8 bool);

macro_rules! serialize_rust_real {
    ( $( $t:ident )* ) => {
        $(
            impl JSSerialize for $t {
                type WireType = $t;

                fn to_wire_type(&self) -> $t {
                    *self
                }

                fn serialize(&self) -> EM_GENERIC_WIRE_TYPE {
                    *self as _
                }
            }
        )*
    }
}

serialize_rust_real!(f64 f32);
