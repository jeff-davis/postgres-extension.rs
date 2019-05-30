
use libc::*;

#[repr(C)]
pub struct varlena {
    pub vl_len_: [c_char; 4],
    pub vl_dat: c_char,
}

pub type text = varlena;
