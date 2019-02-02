#[macro_use]
extern crate postgres_extension;
#[macro_use]
extern crate postgres_extension_macro;

use std::ffi::{CStr};
use postgres_extension::utils::elog::*;
use postgres_extension::fmgr::*;
use postgres_extension::postgres::*;
use postgres_extension::executor::spi::*;

pg_module_magic!();

pg_function_info_v1!(udf_spi);
fn udf_spi(_fcinfo: FunctionCallInfo) -> Datum {
    let spi = spi_connect();
    let res = spi.execute("select * from foo", false).unwrap();
    elog!(NOTICE, "status: {}", res.status);
    let tupdesc = res.tupdesc();
    let natts = tupdesc.natts;
    let tuples = res.tuples();
    for tuple in tuples {
        let mut s = String::new();
        for column in 1..=natts {
            let val = spi_getvalue(tuple, tupdesc, column);
            //let val = pgval_cstr.to_str().unwrap();
            s.push_str(&format!("{}, ", val));
        }
        elog!(NOTICE, "[ {} ]", s);
    }
    return Int32GetDatum(1);
}
