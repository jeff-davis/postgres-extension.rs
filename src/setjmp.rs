use libc::*;

#[repr(C)]
pub struct jmp_buf {
    __jmpbuf: [i64; 8],
    __mask_was_saved: c_int,
    __saved_mask: sigset_t,
}

#[repr(C)]
pub struct sigjmp_buf {
    __jmpbuf: [i64; 9],
    __mask_was_saved: c_int,
    __saved_mask: sigset_t,
}

extern "C" {
    #[link_name="setjmp"]
    pub fn setjmp(env: *mut jmp_buf) -> c_int;
    #[link_name="__sigsetjmp"]
    pub fn sigsetjmp(env: *mut sigjmp_buf, savesigs: c_int) -> c_int;
    pub fn longjmp(env: *mut jmp_buf, val: c_int) -> c_void;
    pub fn siglongjmp(env: *mut sigjmp_buf, val: c_int) -> c_void;
}
