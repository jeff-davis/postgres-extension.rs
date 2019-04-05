use libc::*;
use crate::postgres::*;

pub const BGWORKER_SHMEM_ACCESS: c_int = 0x0001;
pub const BGWORKER_BACKEND_DATABASE_CONNECTION: c_int = 0x0002;
pub const BGW_MAXLEN: usize = 96;
pub const BGW_EXTRALEN: usize = 128;

#[repr(C)]
pub enum BgWorkerStartTime {
    BgWorkerStart_PostmasterStart,
    BgWorkerStart_ConsistentState,
    BgWorkerStart_RecoveryFinished,
}

#[repr(C)]
pub struct BackgroundWorker {
    pub bgw_name: [c_char; BGW_MAXLEN],
    pub bgw_type: [c_char; BGW_MAXLEN],
    pub bgw_flags: c_int,
    pub bgw_start_time: BgWorkerStartTime,
    pub bgw_restart_time: c_int,   /* in seconds, or BGW_NEVER_RESTART */
    pub bgw_library_name: [c_char; BGW_MAXLEN],
    pub bgw_function_name: [c_char; BGW_MAXLEN],
    pub bgw_main_arg: Datum,
    pub bgw_extra: [c_char; BGW_EXTRALEN],
    pub bgw_notify_pid: pid_t, /* SIGUSR1 this backend on start/stop */
}

extern "C" {
    pub fn RegisterBackgroundWorker(worker: &BackgroundWorker);
    pub fn BackgroundWorkerBlockSignals();
    pub fn BackgroundWorkerUnblockSignals();
}
