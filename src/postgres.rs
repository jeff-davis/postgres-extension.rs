
pub use crate::postgres_ext::*;

pub type Datum = usize;

pub fn Int32GetDatum(val : i32) -> Datum {
    return val as Datum;
}

pub fn DatumGetInt32(val : Datum) -> i32 {
    return val as i32;
}
