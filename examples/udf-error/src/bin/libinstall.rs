use std::process::Command;

fn pg_config(s: &str) -> String {
    String::from_utf8(Command::new("pg_config")
        .args(&[s])
        .output()
        .expect("failed to run pg_config")
        .stdout).unwrap().trim().to_string()
}

fn pkgname_to_libname(name: String) -> String {
    let libname = name.replace("-","_");
    if cfg!(target_os = "windows") {
        format!("{}.dll", libname)
    } else if cfg!(target_os = "macos") {
        format!("lib{}.dylib", libname)
    } else {
        format!("lib{}.so", libname)
    }
}

fn main() {
    let pkglibdir = pg_config("--pkglibdir");
    let pwd = std::env::var("PWD").unwrap();
    let pkgname = std::env::var("CARGO_PKG_NAME").unwrap();
    let libname = pkgname_to_libname(pkgname);
    let profile = if cfg!(debug_assertions) { "debug" } else { "release" };
    let lib_path = format!("{}/target/{}/{}", pwd, profile, libname);
    println!("installing to: {}", pkglibdir);
    println!("library path: {}", lib_path);
}
