extern crate cc;
extern crate pkg_config;
use std::env;

fn main() {
    println!("cargo:rerun-if-env-changed=SQLITE3_LIB_DIR");
    if let Ok(lib_path) = env::var("SQLITE3_LIB_DIR") {
        println!("cargo:rerun-if-changed={lib_path}");
        println!("cargo:rustc-link-lib=static=sqlite3");
        println!("cargo:rustc-link-search={lib_path}");
    } else if cfg!(feature = "bundled") || pkg_config::find_library("sqlite3").is_err() {
        cc::Build::new()
            .file("source/sqlite3.c")
            .compile("libsqlite3.a");
    }
}
