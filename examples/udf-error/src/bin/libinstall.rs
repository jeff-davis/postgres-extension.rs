extern crate pg_config;
extern crate cdylib_plugin;

fn main() {
    let cfg = pg_config::pg_config();
    let lib_path = cdylib_plugin::cdylib_path();
    dbg!(&cfg);
    println!("installing to: {}", cfg.pkglibdir);
    println!("library path: {}", lib_path);
}
