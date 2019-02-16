#[macro_use]
extern crate postgres_extension;
#[macro_use]
extern crate postgres_extension_macro;

use postgres_extension::fmgr::*;
use postgres_extension::postgres::*;
use postgres_extension::utils::elog::*;

pg_module_magic!();

#[pg_export(V1)]
fn udf_error(fcinfo: FunctionCallInfo) -> Datum {
    ereport!(ERROR,
             (errcode(ERRCODE_EXTERNAL_ROUTINE_EXCEPTION),
              errmsg("test error: {}", ERRCODE_EXTERNAL_ROUTINE_EXCEPTION),
              errhint("asdf"),
              errdetail("{} {} {}",1,2,3))
    );
    return 1;
}

#[pg_export(V1)]
fn udf_panic(fcinfo: FunctionCallInfo) -> Datum {
    panic!("udf panic")
}

#[pg_export(V1)]
fn foo(fcinfo: FunctionCallInfo) -> Datum {
    let mut v = vec![1,2];
    let i = 1; //DatumGetInt32(pg_getarg(fcinfo,0).unwrap());
    if i >= 1000 {
        udf_panic(fcinfo);
    }
    else if i > 100 {
        elog!(ERROR, "number {} is too big!", i);
    }
    else if i > 10 {
        elog!(WARNING, "number {} is big", i);
    }
    v.push(i);
    return Int32GetDatum(v[2] + 1);
}
