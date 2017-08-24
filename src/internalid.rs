pub trait InternalID {
    fn internal_id() -> char;
}

impl InternalID for () {
    fn internal_id() -> char {
        'v'
    }
}

macro_rules! internalid_rust {
    ( $sign: expr, $( $t:ident )* ) => {
        $(
            impl InternalID for $t {
                fn internal_id() -> char {
                    $sign
                }
            }
        )*
    }
}

internalid_rust!('i', isize i32 i16 i8 usize u32 u16 u8 bool);
internalid_rust!('d', f64 f32);
