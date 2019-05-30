#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]

// dependencies
extern crate libc;

// rust modules
pub mod rust_utils;
pub mod setjmp;

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
