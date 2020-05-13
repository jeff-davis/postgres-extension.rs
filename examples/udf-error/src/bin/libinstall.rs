extern crate postgres_util;
extern crate cdylib_plugin;

fn main() {
    let postgres = postgres_util::postgres();
    let lib_path = cdylib_plugin::cdylib_path();
    dbg!(&postgres);
    println!("installing to: {}", postgres["PKGLIBDIR"]);
    println!("library path: {}", lib_path.to_str().unwrap());
}
