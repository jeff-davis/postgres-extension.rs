
#[repr(C)]
pub struct MemoryContextData { _unused: [u8; 0] } // opaque

pub type MemoryContext = *mut MemoryContextData;

extern "C" {
    pub static CurrentMemoryContext: MemoryContext;
}

pub mod c {
    use crate::utils::memutils::*;
    extern "C" {
        pub fn MemoryContextAlloc(context: MemoryContext, size: usize) -> *mut u8;
        pub fn pfree(ptr: *mut u8) -> ();
    }
}

