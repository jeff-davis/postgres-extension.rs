
// includes
use libc::{c_int,c_void};
use std::marker::PhantomData;

// constants
pub const FUNC_MAX_ARGS: c_int = 100;

// globals
pub static PG_MODULE_MAGIC_DATA: Pg_magic_struct =
    Pg_magic_struct {
        len: 28, // TODO: size_of::<Pg_magic_struct>() as c_int,
        version: ::PG_VERSION_NUM / 100,
        funcmaxargs: FUNC_MAX_ARGS,
        indexmaxkeys: ::INDEX_MAX_KEYS,
        nameddatalen: ::NAMEDATALEN,
        float4byval: ::FLOAT4PASSBYVAL,
        float8byval: ::FLOAT8PASSBYVAL
    };

pub static PG_FUNCTION_INFO_V1_DATA : Pg_finfo_record =
    Pg_finfo_record { api_version : 1 };

// types

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
struct FunctionCallInfoData {
    fl_info: *mut c_void,
    context: fmNodePtr,
    result_info: fmNodePtr,
    fn_collation: c_void,
    is_null: bool,
    nargs: u16,
    arg: [Datum; FUNC_MAX_ARGS as usize],
    argnull: [bool; FUNC_MAX_ARGS as usize]
}

#[allow(dead_code)]
pub struct FunctionCallInfo<'a> {
    ptr: *mut FunctionCallInfoData,
    marker: PhantomData<&'a FunctionCallInfoData>
}

pub type Datum = usize;

pub fn pg_getarg(fcinfo: FunctionCallInfo, arg_num: usize) -> Option<Datum> {
    unsafe {
        if (*fcinfo.ptr).argnull[arg_num] {
            None
        } else {
            Some((*fcinfo.ptr).arg[arg_num])
        }
    }
}


