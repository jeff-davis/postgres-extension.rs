#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]

// dependencies
extern crate libc;
pub extern crate cee_scape;

// rust modules
#[macro_use]
pub mod rust_utils;

// PG modules
pub mod access;
pub mod c;
pub mod executor;
pub mod fmgr;
pub mod pg_config;
pub mod postmaster;
pub mod postgres;
pub mod postgres_ext;
pub mod utils;

#[global_allocator]
static ALLOCATOR: rust_utils::PostgresAllocator = rust_utils::PostgresAllocator;
