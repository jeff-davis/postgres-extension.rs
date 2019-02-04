#![macro_use]
#![allow(non_snake_case)]

use libc::*;
use std::ffi::CString;
use crate::setjmp::*;

#[repr(C)]
pub struct ErrorContextCallback {
    previous: *mut ErrorContextCallback,
    callback: extern fn(arg: *mut c_void),
    arg: *mut c_void,
}

#[macro_export]
macro_rules! elog {
    ($elevel:expr, $fmt:expr, $($args:tt)*) => {
        postgres_extension::utils::elog::elog_internal(
            file!(), line!(), $elevel, &format!($fmt, $($args)*));
    };
}

extern {
    fn elog_start(filename : *const c_char, lineno : c_int, funcname : *const c_char ) -> ();
    fn elog_finish(elevel : c_int, fmt : *const c_char, ...) -> ();
    pub fn pg_re_throw() -> ();
}

pub const DEBUG5  : i32 = 10;
pub const DEBUG4  : i32 = 11;
pub const DEBUG3  : i32 = 12;
pub const DEBUG2  : i32 = 13;
pub const DEBUG1  : i32 = 14;
pub const LOG     : i32 = 15;
pub const INFO    : i32 = 17;
pub const NOTICE  : i32 = 18;
pub const WARNING : i32 = 19;
pub const ERROR   : i32 = 20;
pub const FATAL   : i32 = 21;
pub const PANIC   : i32 = 22;
pub fn elog_internal(filename: &str, lineno: u32, elevel: i32, fmt: &str) -> () {
    let cfilename = CString::new(filename).unwrap().as_ptr();
    let clineno = lineno as c_int;
    /* rust doesn't have a macro to provide the current function name */
    let cfuncname = std::ptr::null::<c_char>();
    let celevel = elevel as c_int;
    let cfmt = CString::new(fmt).unwrap();

    unsafe {
        elog_start(cfilename, clineno, cfuncname);
        elog_finish(celevel, cfmt.as_ptr());
    }
}

extern "C" {
    #[allow(dead_code)]
    pub static mut PG_exception_stack: *mut sigjmp_buf;
    pub static mut error_context_stack: *mut ErrorContextCallback;
}

pub struct PgError;

#[macro_export]
macro_rules! longjmp_panic {
    ($e:expr) => {
        let retval;
        unsafe {
            use postgres_extension::utils::elog
                ::{PG_exception_stack,
                   error_context_stack,
                   PgError};
            use postgres_extension::setjmp::{sigsetjmp,sigjmp_buf};
            let save_exception_stack: *mut sigjmp_buf = PG_exception_stack;
            let save_context_stack: *mut ErrorContextCallback = error_context_stack;
            let mut local_sigjmp_buf: sigjmp_buf = std::mem::uninitialized();
            if sigsetjmp(&mut local_sigjmp_buf, 0) == 0 {
                PG_exception_stack = &mut local_sigjmp_buf;
                retval = $e;
            } else {
                PG_exception_stack = save_exception_stack;
                error_context_stack = save_context_stack;
                panic!(PgError);
            }
            PG_exception_stack = save_exception_stack;
            error_context_stack = save_context_stack;
        }
        retval
    }
}
