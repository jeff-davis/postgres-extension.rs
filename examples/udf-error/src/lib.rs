#[macro_use]
extern crate postgres_extension as pgx;
#[macro_use]
extern crate postgres_extension_macro;

use pgx::fmgr::*;
use pgx::postgres::*;
use pgx::utils::elog::*;

pg_module_magic!();

extern "C" {
    fn int4div(fcinfo: FunctionCallInfo) -> Datum;
    fn DirectFunctionCall2Coll(func: PGFunction, collation: Oid,
                               arg1: Datum, arg2: Datum) -> Datum;
}

struct Foo {
    s: &'static str,
}

impl Drop for Foo {
    fn drop(&mut self) {
        eprintln!("destructor called: {}", self.s);
    }
}

#[pg_export(V1)]
fn udf_divzero(_fcinfo: FunctionCallInfo) -> Datum {
    let _foo = Foo {s: "udf_divzero"};
    longjmp_panic!(
        DirectFunctionCall2Coll(int4div, InvalidOid, 1, 0)
    );
    return 0;
}

#[pg_export(V1)]
fn udf_error(_fcinfo: FunctionCallInfo) -> Datum {
    let _foo = Foo {s: "udf_error"};
    ereport!(ERROR,
             (errcode(ERRCODE_EXTERNAL_ROUTINE_EXCEPTION),
              errmsg("test error: {}", ERRCODE_EXTERNAL_ROUTINE_EXCEPTION),
              errhint("asdf"),
              errdetail("{} {} {}",1,2,3))
    );
    return 1;
}

#[pg_export(V1)]
fn udf_panic(_fcinfo: FunctionCallInfo) -> Datum {
    let _foo = Foo {s: "udf_panic"};
    panic!("udf panic")
}
