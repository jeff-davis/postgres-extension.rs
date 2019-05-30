
pub use crate::postgres_ext::*;

pub type Datum = usize;

#[repr(C)]
pub struct NullableDatum {
    pub value: Datum,
    pub isnull: bool,
}

pub fn Int32GetDatum(val : i32) -> Datum {
    return val as Datum;
}

pub fn DatumGetInt32(val : Datum) -> i32 {
    return val as i32;
}
