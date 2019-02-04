#[macro_use]
extern crate postgres_extension;
#[macro_use]
extern crate postgres_extension_macro;

use postgres_extension::fmgr::*;
use postgres_extension::postgres::*;
use postgres_extension::utils::elog::*;

pg_module_magic!();

pg_function_info_v1!(udf_error);
fn udf_error(_fcinfo: FunctionCallInfo) -> Datum {
    longjmp_panic!(elog_internal(file!(), line!(), ERROR, "test error"));
    return 1;
}

pg_function_info_v1!(udf_panic);
fn udf_panic(_fcinfo: FunctionCallInfo) -> Datum {
    panic!("udf panic")
}
