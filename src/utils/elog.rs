#![macro_use]
#![allow(non_snake_case)]

use libc::*;
use crate::setjmp::*;

#[repr(C)]
pub struct ErrorContextCallback {
    previous: *mut ErrorContextCallback,
    callback: extern fn(arg: *mut c_void),
    arg: *mut c_void,
}

#[macro_export]
macro_rules! elog {
    ($elevel:expr, $($args:expr),+) => {
        ereport!($elevel, (errmsg($($args),+)));
    };
}

#[macro_export]
macro_rules! ereport {
    ($elevel:expr, ($($kind:tt($($args:expr),*)),+)) => {
        unsafe {
            use postgres_extension::utils::elog::{
                PanicErrfinish, ERROR,
                pg_errstart,errfinish,
                errmsg,errhint,errcode,errdetail};

            if pg_errstart($elevel, file!(), line!()) {

                $(
                    pg_errfmt!($kind,$($args),+);
                )+

                if $elevel >= ERROR {
                    panic!(PanicErrfinish);
                } else {
                    errfinish(0);
                }
            }
        }
    }
}

#[macro_export]
macro_rules! pg_errfmt {
    (errcode, $arg:expr) => {
        errcode($arg);
    };
    ($kind:tt, $($args:expr),+) => {
        let s: &str = &format!($($args),+);
        let cstring = std::ffi::CString::new(s).unwrap();
        $kind(cstring.as_ptr());
    }
}

type c_bool = c_char;

pub unsafe fn pg_errstart(elevel: i32, _filename: &str, lineno: u32) -> bool {
    //TODO: find a way to make a constant c string out of file!()
    let cfilename = std::ptr::null::<c_char>();
    let clineno = lineno as c_int;
    let cfuncname = std::ptr::null::<c_char>();
    let cdomain = std::ptr::null::<c_char>();

    let result = errstart(elevel, cfilename,
                          clineno, cfuncname, cdomain);

    if result == 0 {
        return false;
    } else {
        return true;
    }
}

extern {
    pub fn elog_start(filename : *const c_char, lineno : c_int, funcname : *const c_char ) -> ();
    pub fn elog_finish(elevel : c_int, fmt : *const c_char, ...) -> ();
    pub fn pg_re_throw() -> !;
    pub fn errstart(elevel: c_int,
                    filename: *const c_char,
                    lineno: c_int,
                    funcname: *const c_char,
                    domain: *const c_char) -> c_bool;
    pub fn errfinish(dummy: c_int, ...);
    pub fn errmsg(fmt: *const c_char, ...) -> c_int;
    pub fn errdetail(fmt: *const c_char, ...) -> c_int;
    pub fn errhint(fmt: *const c_char, ...) -> c_int;
    pub fn errcode(sqlerrcode: c_int) -> c_int;
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

pub const TEXTDOMAIN: *const c_char = std::ptr::null::<c_char>();

const fn pgsixbit(ch: char) -> u32 {
    return ((ch as u32) - ('0' as u32)) & 0x3f;
}
const fn make_sqlstate(ch1: char, ch2: char, ch3: char, ch4: char, ch5: char) -> i32 {
    return (
        (pgsixbit(ch1) << 0) +
        (pgsixbit(ch2) << 6) +
        (pgsixbit(ch3) << 12) +
        (pgsixbit(ch4) << 18) +
        (pgsixbit(ch5) << 24)
    ) as i32;
}

pub const ERRCODE_EXTERNAL_ROUTINE_EXCEPTION: c_int = make_sqlstate('3','8','0','0','0');

extern "C" {
    #[allow(dead_code)]
    pub static mut PG_exception_stack: *mut sigjmp_buf;
    pub static mut error_context_stack: *mut ErrorContextCallback;
}

pub struct PanicErrfinish;
pub struct PanicReThrow;

