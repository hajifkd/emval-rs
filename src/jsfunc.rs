extern crate emval_sys;


use emval_sys::*;

use std;
use std::sync::{Once, ONCE_INIT};

use jsobj::*;
use jsserialize::*;
use jsid::*;
use jsdeserialize::*;

pub trait JSFunc {
    fn to_object(self) -> JSObj;
}

macro_rules! count_idents {
    ( $_h:ident ) => {0};
    ( $_h:ident, $( $tail:ident ),*) => {1 + count_idents!($( $tail ),*)};
}

static mut TEMP_ID: usize = 0;

macro_rules! jsfunc_for {
    ( $signature:ident, $( $vals:ident => $args:ident ),* ) => {
        impl<RET, $( $args ),*> JSFunc for Box<Box<Fn($( $args ),*) -> RET>>
            where RET: JSSerialize, $( $args: JSDeserialize ),* {

            fn to_object(self) -> JSObj {
                pub extern fn helper_helper<RETP: JSSerialize, $( $args: JSDeserialize ),*>(hptr: usize, cptr: usize, $( $vals:usize ),*) -> EM_GENERIC_WIRE_TYPE {
                    let helper: Box<Box<Fn(usize, $( $args ),*) -> RETP>> = unsafe { Box::from_raw(hptr as _) };
                    let result = helper(cptr, $( $args::from_jsobj_id($vals as _) ),*);

                    unsafe {
                        $( _emval_decref($vals as _); )*
                    }

                    std::mem::forget(helper);

                    result.serialize()
                }

                // TODO make unique name for each realization of the templates
                let helper_name = unsafe {
                    format!("{}{}", concat!("rust_helper_", stringify!($signature)), TEMP_ID)
                };
                unsafe { TEMP_ID = TEMP_ID + 1; }
                static REGISTER: Once = ONCE_INIT;
                REGISTER.call_once(|| {
                    let mut arglist = vec![JSObj::id(); count_idents!($( $args ),*) + 2];
                    arglist[0] = RET::id();
                    arglist[1] = usize::id();

                    let helper = |cptr: usize, $( $vals:$args ),*| {
                        let closure: Box<Box<Fn($( $args ),*) -> RET>> = unsafe { Box::from_raw(cptr as _) };
                        let result = closure($( $vals ),*);

                        std::mem::forget(closure);

                        result
                    };

                    let helper_boxed = Box::new(Box::new(helper));

                    unsafe {
                        _embind_register_function(format!("{}\0", helper_name).as_ptr() as _,
                                                  count_idents!($( $args ),*) + 2,
                                                  arglist.as_ptr() as _,
                                                  concat!(stringify!($signature), '\0').as_ptr() as _,
                                                  helper_helper::<RET, $( $args ),*> as _,
                                                  Box::into_raw(helper_boxed) as _);
                    }
                });

                let self_ptr = Box::into_raw(self);
                let module = JSObj::global("Module");
                let helper = module.get_prop(&helper_name);
                let args = Args::new(vec![JSObj::id(), usize::id()], vec![module.serialize(), (self_ptr as usize).serialize()]);
                helper.call_prop("bind", args)
            }
        }
    }
}

/*
 * putStrLn $ foldr ((++) . ("\n" ++)) "" $ map (("jsfunc_for!(" ++) . (++ ");")) $ map (\x -> (replicate x 'i') ++ foldr (++) "" [", a" ++ show y ++ " => A" ++ show y | y <- [1..x]]) [1..5]
 */
jsfunc_for!(ii, a1 => A1);
/*
jsfunc_for!(ii, a1 => A1, a2 => A2);
jsfunc_for!(iii, a1 => A1, a2 => A2, a3 => A3);
jsfunc_for!(iiii, a1 => A1, a2 => A2, a3 => A3, a4 => A4);
jsfunc_for!(iiiii, a1 => A1, a2 => A2, a3 => A3, a4 => A4, a5 => A5);
jsfunc_for!(iiiiii, a1 => A1, a2 => A2, a3 => A3, a4 => A4, a5 => A5, a6 => A6);
jsfunc_for!(iiiiiii, a1 => A1, a2 => A2, a3 => A3, a4 => A4, a5 => A5, a6 => A6, a7 => A7);
jsfunc_for!(iiiiiiii, a1 => A1, a2 => A2, a3 => A3, a4 => A4, a5 => A5, a6 => A6, a7 => A7, a8 => A8);
jsfunc_for!(iiiiiiiii, a1 => A1, a2 => A2, a3 => A3, a4 => A4, a5 => A5, a6 => A6, a7 => A7, a8 => A8, a9 => A9);
jsfunc_for!(iiiiiiiiii, a1 => A1, a2 => A2, a3 => A3, a4 => A4, a5 => A5, a6 => A6, a7 => A7, a8 => A8, a9 => A9, a10 => A10);
*/
/*
 * TODO vi type
jsfunc_for!(v, a1 => A1);
jsfunc_for!(vi, a1 => A1, a2 => A2);
jsfunc_for!(vii, a1 => A1, a2 => A2, a3 => A3);
jsfunc_for!(viii, a1 => A1, a2 => A2, a3 => A3, a4 => A4);
jsfunc_for!(viiii, a1 => A1, a2 => A2, a3 => A3, a4 => A4, a5 => A5);
jsfunc_for!(viiiii, a1 => A1, a2 => A2, a3 => A3, a4 => A4, a5 => A5, a6 => A6);
jsfunc_for!(viiiiii, a1 => A1, a2 => A2, a3 => A3, a4 => A4, a5 => A5, a6 => A6, a7 => A7);
jsfunc_for!(viiiiiii, a1 => A1, a2 => A2, a3 => A3, a4 => A4, a5 => A5, a6 => A6, a7 => A7, a8 => A8);
jsfunc_for!(viiiiiiii, a1 => A1, a2 => A2, a3 => A3, a4 => A4, a5 => A5, a6 => A6, a7 => A7, a8 => A8, a9 => A9);
jsfunc_for!(viiiiiiiii, a1 => A1, a2 => A2, a3 => A3, a4 => A4, a5 => A5, a6 => A6, a7 => A7, a8 => A8, a9 => A9, a10 => A10);
*/
