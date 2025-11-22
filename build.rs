// build.rs
use std::env;

fn main() {
    let inc = env::var("MSPACK_INCDIR").unwrap_or_default();
    let lib = env::var("MSPACK_LIBDIR").unwrap_or_default();

    if !inc.is_empty() {
        println!("cargo:include={}", inc);
    }
    if !lib.is_empty() {
        println!("cargo:rustc-link-search=native={}", lib);
    }
    // static link to the mspack archive we built via CMake
    println!("cargo:rustc-link-lib=static=mspack");

    println!("cargo:rerun-if-env-changed=MSPACK_INCDIR");
    println!("cargo:rerun-if-env-changed=MSPACK_LIBDIR");
}
