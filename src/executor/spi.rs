
use std::marker::PhantomData;
use std::ffi::{CString,CStr};
use crate::utils::memutils::MemoryContext;
use libc::*;

/* Plans are opaque structs for standard users of SPI */
pub type SPIPlanPtr = *mut _SPI_plan;
pub type HeapTuple = *mut HeapTupleData;
pub type TupleDesc = *mut TupleDescData;
pub type SubTransactionId = u32;

#[repr(C)]
pub struct TupleDescData { pub natts: c_int, _private: [u8; 0] }
#[repr(C)]
pub struct HeapTupleData { _private: [u8; 0] }
#[repr(C)]
pub struct _SPI_plan { _private: [u8; 0] }
#[repr(C)]
pub struct slist_node { _private: [u8; 0] }

#[repr(C)]
pub struct SPITupleTable
{
    tuptabcxt: MemoryContext,	/* memory context of result table */
    alloced: u64,		/* # of alloced vals */
    free: u64,			/* # of free vals */
    pub tupdesc: TupleDesc,		/* tuple descriptor */
    pub vals: *mut HeapTuple,			/* tuples */
    next: *mut slist_node,			/* link for internal bookkeeping */
    subid: SubTransactionId,		/* subxact in which tuptable was created */
}

pub const SPI_ERROR_CONNECT: i32 = -1;
pub const SPI_ERROR_COPY: i32 = -2;
pub const SPI_ERROR_OPUNKNOWN: i32 = -3;
pub const SPI_ERROR_UNCONNECTED: i32 = -4;
pub const SPI_ERROR_CURSOR: i32 = -5;	/* not used anymore */
pub const SPI_ERROR_ARGUMENT: i32 = -6;
pub const SPI_ERROR_PARAM: i32 = -7;
pub const SPI_ERROR_TRANSACTION: i32 = -8;
pub const SPI_ERROR_NOATTRIBUTE: i32 = -9;
pub const SPI_ERROR_NOOUTFUNC: i32 = -10;
pub const SPI_ERROR_TYPUNKNOWN: i32 = -11;
pub const SPI_ERROR_REL_DUPLICATE: i32 = -12;
pub const SPI_ERROR_REL_NOT_FOUND: i32 = -13;

pub const SPI_OK_CONNECT: i32 = 1;
pub const SPI_OK_FINISH: i32 = 2;
pub const SPI_OK_FETCH: i32 = 3;
pub const SPI_OK_UTILITY: i32 = 4;
pub const SPI_OK_SELECT: i32 = 5;
pub const SPI_OK_SELINTO: i32 = 6;
pub const SPI_OK_INSERT: i32 = 7;
pub const SPI_OK_DELETE: i32 = 8;
pub const SPI_OK_UPDATE: i32 = 9;
pub const SPI_OK_CURSOR: i32 = 10;
pub const SPI_OK_INSERT_RETURNING: i32 = 11;
pub const SPI_OK_DELETE_RETURNING: i32 = 12;
pub const SPI_OK_UPDATE_RETURNING: i32 = 13;
pub const SPI_OK_REWRITTEN: i32 = 14;
pub const SPI_OK_REL_REGISTER: i32 = 15;
pub const SPI_OK_REL_UNREGISTER: i32 = 16;
pub const SPI_OK_TD_REGISTER: i32 = 17;
pub const SPI_OPT_NONATOMIC: i32 = (1 << 0);

pub mod c {
    use libc::*;
    use super::*;
    extern {
        pub static SPI_processed: u64;
        pub static SPI_tuptable: *mut crate::executor::spi::SPITupleTable;
        pub static SPI_result: c_int;

        pub fn SPI_connect() -> c_int;
        pub fn SPI_connect_ext(options: c_int) -> c_int;
        pub fn SPI_finish() -> c_int;
        pub fn SPI_execute(src: *const c_char, read_only: bool,
                           tcount: c_long) -> c_int;
        /*
        pub fn SPI_execute_plan(SPIPlanPtr plan, Datum *Values, const char *Nulls,
	bool read_only, long tcount) -> c_int;
        pub fn SPI_execute_plan_with_paramlist(SPIPlanPtr plan,
	ParamListInfo params,
	bool read_only, long tcount) -> c_int;
         */
        pub fn SPI_exec(src: *const c_char, tcount: c_long) -> c_int;
        /*
        pub fn SPI_execp(SPIPlanPtr plan, Datum *Values, const char *Nulls,
	long tcount) -> c_int;
        pub fn SPI_execute_snapshot(SPIPlanPtr plan,
	Datum *Values, const char *Nulls,
	Snapshot snapshot,
	Snapshot crosscheck_snapshot,
	bool read_only, bool fire_triggers,
        tcount: c_long) -> c_int;
        pub fn SPI_execute_with_args(const char *src,
	int nargs, Oid *argtypes,
	Datum *Values, const char *Nulls,
	bool read_only, long tcount) -> int;
        pub fn SPI_prepare(const char *src, int nargs, Oid *argtypes) -> SPIPlanPtr;
        pub fn SPI_prepare_cursor(const char *src, int nargs, Oid *argtypes,
	int cursorOptions) -> SPIPlanPtr;
        pub fn SPI_prepare_params(const char *src,
	ParserSetupHook parserSetup,
	void *parserSetupArg,
	int cursorOptions) -> SPIPlanPtr;
        pub fn SPI_keepplan(SPIPlanPtr plan) -> int;
        pub fn SPI_saveplan(SPIPlanPtr plan) -> SPIPlanPtr;
        pub fn SPI_freeplan(SPIPlanPtr plan) -> int;

        pub fn SPI_getargtypeid(SPIPlanPtr plan, int argIndex) -> Oid;
        pub fn SPI_getargcount(SPIPlanPtr plan) -> int;
        pub fn SPI_is_cursor_plan(SPIPlanPtr plan) -> bool;
        pub fn SPI_plan_is_valid(SPIPlanPtr plan) -> bool;
        pub fn SPI_result_code_string(int code) -> *const c_char;

        pub fn SPI_plan_get_plan_sources(SPIPlanPtr plan) -> *mut List;
        pub fn SPI_plan_get_cached_plan(SPIPlanPtr plan) -> *mut CachedPlan;

        pub fn SPI_copytuple(HeapTuple tuple) -> HeapTuple;
        pub fn SPI_returntuple(HeapTuple tuple, TupleDesc tupdesc) -> HeapTupleHeader;
        pub fn SPI_modifytuple(Relation rel, HeapTuple tuple, int natts,
	int *attnum, Datum *Values, const char *Nulls) -> HeapTuple;
        pub fn SPI_fnumber(TupleDesc tupdesc, const char *fname) -> int;
        pub fn SPI_fname(TupleDesc tupdesc, int fnumber) -> *const c_char;
         */
        pub fn SPI_getvalue(tuple: HeapTuple, tupdesc: TupleDesc,
                            fnumber: c_int) -> *const c_char;
        /*
        pub fn SPI_getbinval(HeapTuple tuple, TupleDesc tupdesc, int fnumber, bool *isnull) -> Datum;
        pub fn SPI_gettype(TupleDesc tupdesc, int fnumber) -> *const c_char
        pub fn SPI_gettypeid(TupleDesc tupdesc, int fnumber) -> Oid;
        pub fn SPI_getrelname(Relation rel) -> *const c_char;
        pub fn SPI_getnspname(Relation rel) -> *const c_char;
        pub fn SPI_palloc(Size size) -> *mut c_void;
        pub fn SPI_repalloc(void *pointer, Size size) -> *mut c_void;
        pub fn SPI_pfree(void *pointer);
        pub fn SPI_datumTransfer(Datum value, bool typByVal, int typLen) -> Datum;
        pub fn SPI_freetuple(HeapTuple pointer) -> void;
        */
        pub fn SPI_freetuptable(tuptable: *mut SPITupleTable);
        /*
        pub fn SPI_cursor_open(const char *name, SPIPlanPtr plan,
	Datum *Values, const char *Nulls, bool read_only) -> Portal;
        pub fn SPI_cursor_open_with_args(const char *name,
	const char *src,
	int nargs, Oid *argtypes,
	Datum *Values, const char *Nulls,
	bool read_only, int cursorOptions) -> Portal;
        pub fn SPI_cursor_open_with_paramlist(const char *name, SPIPlanPtr plan,
	ParamListInfo params, bool read_only) -> Portal;
        pub fn SPI_cursor_find(const char *name) -> Portal;
        pub fn SPI_cursor_fetch(Portal portal, bool forward, long count) -> void;
        pub fn SPI_cursor_move(Portal portal, bool forward, long count) -> void;
        pub fn SPI_scroll_cursor_fetch(Portal, FetchDirection direction, long count) -> void;
        pub fn SPI_scroll_cursor_move(Portal, FetchDirection direction, long count) -> void;
        pub fn SPI_cursor_close(Portal portal) -> void;

        pub fn SPI_register_relation(EphemeralNamedRelation enr) -> int;
        pub fn SPI_unregister_relation(const char *name) -> int;
        pub fn SPI_register_trigger_data(tdata: *mut TriggerData) -> c_int;

        pub fn SPI_start_transaction();
        pub fn SPI_commit();
        pub fn SPI_rollback();

        pub fn SPICleanup();
        pub fn AtEOXact_SPI(isCommit: bool);
        pub fn AtEOSubXact_SPI(isCommit: bool, mySubid: SubTransactionId);
        pub fn SPI_inside_nonatomic_context() -> bool;
         */
    }
}


/*

  let spi = spi_connect(options);
  let res1 = spi.exec("select * from foo");
  let res2 = spi.exec("select * from bar");
  spi.freeresult(res2);
  let res3 = spi.exec("select * from baz");
  drop(spi);
 */

pub struct SPIConnection;

pub struct SPIResult<'a> {
    pub status: i32,
    processed: u64,
    tuptable: *mut SPITupleTable,
    phantom: PhantomData<&'a SPITupleTable>,
}

impl SPIConnection {
    pub fn execute(&self, query: &str, readonly: bool) -> Result<SPIResult,i32> {
        let query_cstring = CString::new(query).unwrap();
        let query_ptr = query_cstring.as_ptr();
        unsafe {
            let status = c::SPI_execute(query_ptr, readonly, 0);
            if status < 0 {
                return Err(status);
            }
            return Ok(SPIResult {
                status: status,
                processed: c::SPI_processed,
                tuptable: c::SPI_tuptable,
                phantom: PhantomData,
            })
        }
    }
}

impl Drop for SPIConnection {
    fn drop(&mut self) {
        unsafe {
            c::SPI_finish();
        }
    }
}

impl<'a> SPIResult<'a> {
    pub fn tuples(&self) -> &'a [&HeapTupleData] {
        unsafe {
            let vals: *const &HeapTupleData = (*self.tuptable).vals
                as *const &HeapTupleData;
            return std::slice::from_raw_parts(vals, self.processed as usize);
        }
    }
    pub fn tupdesc(&self) -> &'a TupleDescData {
        unsafe {
            return &*(*self.tuptable).tupdesc;
        }
    }
}

impl<'a> Drop for SPIResult<'a> {
    fn drop(&mut self) {
        unsafe {
            c::SPI_freetuptable(self.tuptable);
        }
    }
}

pub fn spi_connect() -> SPIConnection {
    unsafe {
        c::SPI_connect();
    }
    return SPIConnection {};
}

pub fn spi_getvalue<'a,'b>(tuple: &'a HeapTupleData,
                           tupdesc: &'b TupleDescData,
                           attno: c_int) -> String {
    unsafe {
        let tuple_ptr: HeapTuple = tuple as *const HeapTupleData
            as *mut HeapTupleData;
        let tupdesc_ptr: TupleDesc = tupdesc as *const TupleDescData
            as *mut TupleDescData;
        let val_ptr = c::SPI_getvalue(tuple_ptr, tupdesc_ptr, attno);
        let val_str = CStr::from_ptr(val_ptr).to_str().unwrap();
        return CString::new(val_str).unwrap().into_string().unwrap();
    };
}
