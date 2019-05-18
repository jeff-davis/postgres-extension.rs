
use libc::*;

#[repr(C)]
pub struct MemoryContextData { _unused: [u8; 0] } // opaque

pub type MemoryContext = *mut MemoryContextData;

extern "C" {
    pub static mut CurrentMemoryContext: MemoryContext;
}

pub const ALLOCSET_DEFAULT_MINSIZE: size_t = 0;
pub const ALLOCSET_DEFAULT_INITSIZE: size_t = (8 * 1024);
pub const ALLOCSET_DEFAULT_MAXSIZE: size_t = (8 * 1024 * 1024);

pub mod c {
    use crate::utils::memutils::*;
    extern "C" {
        pub fn MemoryContextAlloc(context: MemoryContext, size: usize) -> *mut u8;
        pub fn AllocSetContextCreateExtended(
            parent: MemoryContext, name: *const c_char, minContextSize: size_t,
            initBlockSize: size_t, maxBlockSize: size_t) -> MemoryContext;
        pub fn pfree(ptr: *mut u8) -> ();
    }
}
