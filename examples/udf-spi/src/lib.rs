#[macro_use]
extern crate postgres_extension as pgx;
#[macro_use]
extern crate postgres_extension_macro;

use pgx::utils::elog::*;
use pgx::fmgr::*;
use pgx::postgres::*;
use pgx::executor::spi::*;

pg_module_magic!();

#[pg_export(V1)]
fn udf_spi(_fcinfo: FunctionCallInfo) -> Datum {
    let spi = spi_connect();
    let result = spi.execute("select * from foo", false).unwrap();
    elog!(NOTICE, "status: {}", result.status);
    for tuple in result.iter() {
        let mut s = String::new();
        for val in tuple.iter() {
            s.push_str(&format!("{}, ", val));
        }
        elog!(NOTICE, "[ {} ]", s);
    }
    return Int32GetDatum(1);
}
