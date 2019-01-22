
// dependencies
extern crate libc;

pub const PG_VERSION_NUM : i32 = 100000;
pub const INDEX_MAX_KEYS : i32 = 32;
pub const NAMEDATALEN : i32 = 64;
pub const FLOAT4PASSBYVAL : i32 = 1;
pub const FLOAT8PASSBYVAL : i32 = 1;

// modules
pub mod fmgr;
