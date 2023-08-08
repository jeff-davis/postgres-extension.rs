
use std::alloc::{GlobalAlloc, Layout};
use std::cmp;
use std::fmt;
use std::io;
use std::io::{Error,ErrorKind,Result};
use std::mem;
use std::ffi::CString;

use crate::utils::elog::*;
use crate::utils::memutils::*;
use crate::utils::memutils::c::*;
use crate::utils::palloc::*;

#[derive(Clone,Copy)]
pub enum PanicType {
    ReThrow,
    Errfinish,
}

// Set up postgres allocator
pub struct PostgresAllocator;

static mut RUST_MEMORY_CONTEXT: MemoryContext = std::ptr::null_mut();
static mut RUST_ERROR_CONTEXT: MemoryContext = std::ptr::null_mut();

#[macro_export]
macro_rules! rust_panic_handler {
    ($e:expr) => {{
        use postgres_extension::utils::elog;
        use postgres_extension::rust_utils;

        rust_utils::init_error_handling();
        let result = std::panic::catch_unwind(|| {
            $e
        });

        let panictype = match result {
            Ok(val) => return val,
            Err(err_any) => rust_utils::handle_panic(err_any),
        };

        unsafe {
            match panictype {
                rust_utils::PanicType::ReThrow => elog::pg_re_throw(),
                rust_utils::PanicType::Errfinish => elog::errfinish(0),
            }
        };

        unreachable!();
    }}
}

#[macro_export]
macro_rules! longjmp_panic {
    ($e:expr) => {
        {
            #[allow(unused_unsafe)]
            unsafe {
                let mut retval = None;
                use std::panic::panic_any;
                use $crate::cee_scape::{call_with_sigsetjmp, SigJmpBufStruct};
                use $crate::utils::elog
                    ::{PG_exception_stack,
                       error_context_stack,
                       ErrorContextCallback};
                use $crate::rust_utils::PanicType;
                let save_exception_stack: *mut SigJmpBufStruct = PG_exception_stack;
                let save_context_stack: *mut ErrorContextCallback = error_context_stack;
                let r = call_with_sigsetjmp(false, |env| {
                    PG_exception_stack = env as *const SigJmpBufStruct as *mut SigJmpBufStruct;
                    retval = Some($e);
                    0
                });

                if r == 0 {
                    PG_exception_stack = save_exception_stack;
                    error_context_stack = save_context_stack;
                    retval.unwrap()
                } else {
                    PG_exception_stack = save_exception_stack;
                    error_context_stack = save_context_stack;
                    panic_any(PanicType::ReThrow);
                }
            }
        }
    }
}

fn init_rust_memory_context() {
    unsafe {
        if RUST_MEMORY_CONTEXT.is_null() {
            RUST_MEMORY_CONTEXT = AllocSetContextCreateInternal(
                TopMemoryContext,
                "Rust Memory Context\0".as_ptr() as *const i8,
                ALLOCSET_DEFAULT_MINSIZE,
                ALLOCSET_DEFAULT_INITSIZE, ALLOCSET_DEFAULT_MAXSIZE);
        }
    }
}

pub fn init_error_handling() {
    unsafe {
        if RUST_ERROR_CONTEXT.is_null() {
            RUST_ERROR_CONTEXT = AllocSetContextCreateInternal(
                TopMemoryContext,
                "Rust Error Context\0".as_ptr() as *const i8,
                ALLOCSET_DEFAULT_MINSIZE,
                ALLOCSET_DEFAULT_INITSIZE, ALLOCSET_DEFAULT_MAXSIZE);

            let oldcontext = MemoryContextSwitchTo(RUST_ERROR_CONTEXT);
            std::panic::set_hook(Box::new(|_| {
                MemoryContextSwitchTo(RUST_ERROR_CONTEXT);
            }));
            MemoryContextSwitchTo(oldcontext);
        }
    }
}

pub fn handle_panic(payload: Box<dyn std::any::Any>) -> PanicType {
    if let Some(panictype) = payload.downcast_ref::<PanicType>() {
        return *panictype
    }

    let panic_message =
        if let Some(s) = payload.downcast_ref::<&str>() {
            s
        } else if let Some(s) = payload.downcast_ref::<String>() {
            &s[..]
        } else {
            "Box<Any>"
        };

    let message = format!("rust panic: {}", panic_message);
    let hint = "find out what rust code caused the panic";
    let detail = "some rust code caused a panic";

    let cmessage = CString::new(message.as_str()).unwrap();
    let chint = CString::new(hint).unwrap();
    let cdetail = CString::new(detail).unwrap();

    unsafe {
        pg_errstart(ERROR, file!(), line!());
        errcode(ERRCODE_EXTERNAL_ROUTINE_EXCEPTION);
        errmsg(cmessage.as_ptr());
        errhint(chint.as_ptr());
        errdetail(cdetail.as_ptr());
    }

    PanicType::Errfinish
}

// implement Write trait for &[i8] (a.k.a. &[c_char])
pub trait Write {
    fn write(&mut self, buf: &[u8]) -> Result<usize>;
    fn flush(&mut self) -> Result<()>;
    fn write_all(&mut self, mut buf: &[u8]) -> Result<()> {
        while !buf.is_empty() {
            match self.write(buf) {
                Ok(0) => return Err(Error::new(ErrorKind::WriteZero,
                                               "failed to write whole buffer")),
                Ok(n) => buf = &buf[n..],
                Err(ref e) if e.kind() == ErrorKind::Interrupted => {}
                Err(e) => return Err(e),
            }
        }
        Ok(())
    }
    fn write_fmt(&mut self, fmt: fmt::Arguments) -> Result<()> {
        // Create a shim which translates a Write to a fmt::Write and saves
        // off I/O errors. instead of discarding them
        struct Adaptor<'a, T: ?Sized + 'a> {
            inner: &'a mut T,
            error: Result<()>,
        }

        impl<'a, T: Write + ?Sized> fmt::Write for Adaptor<'a, T> {
            fn write_str(&mut self, s: &str) -> fmt::Result {
                match self.inner.write_all(s.as_bytes()) {
                    Ok(()) => Ok(()),
                    Err(e) => {
                        self.error = Err(e);
                        Err(fmt::Error)
                    }
                }
            }
        }

        let mut output = Adaptor { inner: self, error: Ok(()) };
        match fmt::write(&mut output, fmt) {
            Ok(()) => Ok(()),
            Err(..) => {
                // check if the error came from the underlying `Write` or not
                if output.error.is_err() {
                    output.error
                } else {
                    Err(Error::new(ErrorKind::Other, "formatter error"))
                }
            }
        }
    }
    fn by_ref(&mut self) -> &mut Self where Self: Sized { self }
}

impl<'a> Write for &'a mut [i8] {
    #[inline]
    fn write(&mut self, data: &[u8]) -> io::Result<usize> {
        let amt = cmp::min(data.len(), self.len());
        let (a, b) = mem::replace(self, &mut []).split_at_mut(amt);
        let a_u8 = unsafe { &mut *(a as *mut [i8] as *mut [u8]) };
        a_u8.copy_from_slice(&data[..amt]);
        *self = b;
        Ok(amt)
    }

    #[inline]
    fn write_all(&mut self, data: &[u8]) -> io::Result<()> {
        if self.write(data)? == data.len() {
            Ok(())
        } else {
            Err(Error::new(ErrorKind::WriteZero, "failed to write whole buffer"))
        }
    }

    #[inline]
    fn flush(&mut self) -> io::Result<()> { Ok(()) }
}

unsafe impl GlobalAlloc for PostgresAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        init_rust_memory_context();
        return MemoryContextAlloc(RUST_MEMORY_CONTEXT, layout.size());
    }
    unsafe fn dealloc(&self, ptr: *mut u8, _layout: Layout) {
        pfree(ptr);
    }
}
