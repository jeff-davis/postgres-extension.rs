
// postgres includes
use crate::pg_config::*;
use crate::postgres::*;

// includes
use libc::{c_int,c_void};
use std::marker::PhantomData;

#[macro_export]
macro_rules! pg_module_magic {
    () => {
        #[no_mangle]
        #[allow(non_snake_case)]
        pub extern fn Pg_magic_func() -> &'static postgres_extension::fmgr::Pg_magic_struct {
            &postgres_extension::fmgr::PG_MODULE_MAGIC_DATA
        }
    }
}

// constants
pub const FUNC_MAX_ARGS: c_int = 100;

// types
type PGFunction = extern fn(FunctionCallInfo) -> Datum;

#[allow(non_camel_case_types)]
type fmNodePtr = *mut c_void;
#[allow(non_camel_case_types)]
pub type fmAggrefPtr = *mut c_void;

#[repr(C)]
pub struct Pg_magic_struct {
    pub len: c_int,
    pub version: c_int,
    pub funcmaxargs: c_int,
    pub indexmaxkeys: c_int,
    pub nameddatalen: c_int,
    pub float4byval: c_int,
    pub float8byval: c_int
}

#[repr(C)]
pub struct Pg_finfo_record {
    pub api_version : c_int,
}

#[repr(C)]
pub struct FmgrInfo {
    fn_addr: PGFunction,
    fn_oid: c_void,
    fn_nargs: u16,
    fn_strict: bool,
    fn_retset: bool,
    fn_stats: u8,
    fn_extra: *mut c_void,
    fn_mcxt: c_void,
    fn_expr: fmNodePtr
}


#[repr(C)]
struct FunctionCallInfoData<'a> {
    flinfo: *mut FmgrInfo,
    context: fmNodePtr,
    result_info: fmNodePtr,
    fncollation: c_void,
    isnull: bool,
    nargs: u16,
    arg: [Datum; FUNC_MAX_ARGS as usize],
    argnull: [bool; FUNC_MAX_ARGS as usize],
    phantom: PhantomData<&'a FmgrInfo>
}

#[repr(C)]
pub struct FunctionCallInfo<'a> {
    ptr: *mut FunctionCallInfoData<'a>,
    phantom: PhantomData<&'a FunctionCallInfoData<'a>>
}

// globals
pub static PG_MODULE_MAGIC_DATA: Pg_magic_struct =
    Pg_magic_struct {
        len: std::mem::size_of::<Pg_magic_struct>() as c_int,
        version: PG_VERSION_NUM / 100,
        funcmaxargs: FUNC_MAX_ARGS,
        indexmaxkeys: INDEX_MAX_KEYS,
        nameddatalen: NAMEDATALEN,
        float4byval: FLOAT4PASSBYVAL,
        float8byval: FLOAT8PASSBYVAL
    };

pub static PG_FUNCTION_INFO_V1_DATA : Pg_finfo_record =
    Pg_finfo_record { api_version : 1 };

#[macro_export]
macro_rules! rust_panic_handler {
    ($e:expr) => {{
        let result = std::panic::catch_unwind(|| {
            $e
        });

        let retval = match result {
            Ok(val) => val,
            Err(err) => {
                use postgres_extension::utils::elog::*;
                unsafe {
                    if POSTGRES_THREW_EXCEPTION {
                        POSTGRES_THREW_EXCEPTION = false;
                        pg_re_throw();
                    } else {
                        elog!(ERROR, "rust panic: {:?}", err);
                    }
                    unreachable!()
                }
            }
        };
        return retval
    }}
}

// functions
pub fn pg_getarg(fcinfo: FunctionCallInfo, arg_num: usize) -> Option<Datum> {
    unsafe {
        if (*fcinfo.ptr).argnull[arg_num] {
            assert!( !(*(*fcinfo.ptr).flinfo).fn_strict );
            None
        } else {
            Some((*fcinfo.ptr).arg[arg_num])
        }
    }
}

