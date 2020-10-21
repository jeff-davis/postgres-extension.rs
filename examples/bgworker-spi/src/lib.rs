#[macro_use]
extern crate postgres_extension as pgx;
extern crate tokio;

use std::ffi::CString;
use std::net::SocketAddr;

use pgx::access::xact::*;
use pgx::executor::spi::*;
use pgx::rust_utils::Write;
use pgx::postmaster::bgworker::*;
use pgx::utils::palloc::*;
use pgx::utils::memutils::*;
use pgx::utils::memutils::c::*;

use tokio::net::{TcpListener, TcpStream};
use tokio::io::{AsyncReadExt,AsyncWriteExt};


static mut BGWORKER_SPI_CONTEXT: MemoryContext = std::ptr::null_mut();

pg_module_magic!();

pub fn process_request(line: String) -> Vec<u8> {
    eprintln!("got request: '{}'\n", line);
    unsafe {
        SetCurrentStatementStartTimestamp();
        StartTransactionCommand();
    }
    let mut s = String::new();
    {
        let spi = spi_connect();
        let catch = std::panic::catch_unwind(|| {
            spi.execute(&line, false).unwrap()
        });
        match catch {
            Ok(res) => {
                for tuple in res.iter() {
                    s.push_str("  (");
                    for val in tuple.iter() {
                        s.push_str(&val);
                        s.push_str(", ");
                    }
                    s.push_str(")\n");
                }
                s.push_str("}\n");
            },
            Err(_) => {
                s.push_str("ERROR\n");
            }
        };
    }

    eprintln!("result: {}", s);
    unsafe {
        let oldcxt = MemoryContextSwitchTo(BGWORKER_SPI_CONTEXT);
        s = s.clone();
        MemoryContextSwitchTo(oldcxt);
    }
    unsafe {
        CommitTransactionCommand();
    }
    return s.into_bytes();
}

#[no_mangle]
pub extern "C" fn _PG_init() {
    let mut worker = BackgroundWorker {
        bgw_name: [0; BGW_MAXLEN],
        bgw_type: [0; BGW_MAXLEN],
        bgw_flags: BGWORKER_SHMEM_ACCESS | BGWORKER_BACKEND_DATABASE_CONNECTION,
        bgw_start_time: BgWorkerStartTime::BgWorkerStart_RecoveryFinished,
        bgw_restart_time: 60,
        bgw_library_name: [0; BGW_MAXLEN],
        bgw_function_name: [0; BGW_MAXLEN],
        bgw_main_arg: 0,
        bgw_extra: [0; BGW_EXTRALEN],
        bgw_notify_pid: 0,
    };
    write!(&mut worker.bgw_name[0..],"rust bgworker name").unwrap();
    write!(&mut worker.bgw_type[0..],"rust bgworker type").unwrap();
    write!(&mut worker.bgw_library_name[0..],"libbgworker_spi").unwrap();
    write!(&mut worker.bgw_function_name[0..],"bgw_main").unwrap();
    unsafe {
        RegisterBackgroundWorker(&worker);
    }
}

async fn process_socket(mut socket: TcpStream) {

    let mut buffer : [u8; 1024] = [0; 1024];
    let (mut reader, mut writer) = socket.split();

    loop {
        let n = reader.read(&mut buffer[..]).await.unwrap();
        let query = std::str::from_utf8(&buffer[0..n]).unwrap();
        let result = process_request(query.to_string());
        let _ = writer.write(&result).await;
    }
}

#[tokio::main(flavor = "current_thread")]
async fn start() {
    let addr = String::from("127.0.0.1:8080").parse::<SocketAddr>().unwrap();
    let listener = TcpListener::bind(&addr).await.unwrap();
    println!("Listening on: {}", addr);

    loop {
        let (socket, _) = listener.accept().await.unwrap();
        tokio::spawn(async move {
            process_socket(socket).await;
        });
    }
}

#[no_mangle]
pub extern "C" fn bgw_main() {

    unsafe {
        BackgroundWorkerUnblockSignals();

        let dbname = CString::new("postgres").unwrap();
        BackgroundWorkerInitializeConnection(dbname.as_ptr(), std::ptr::null(), 0);

        let cxt_name = CString::new("bgworker-spi context").unwrap();
        BGWORKER_SPI_CONTEXT = AllocSetContextCreateInternal(
            CurrentMemoryContext, cxt_name.as_ptr(),
            ALLOCSET_DEFAULT_MINSIZE,
            ALLOCSET_DEFAULT_INITSIZE, ALLOCSET_DEFAULT_MAXSIZE);
    }

    start();
}
