extern crate bindgen;

use std::env;
use std::path::PathBuf;

fn include_library(path: &str) -> String {
    let libraries_dir = env::current_dir()
        .unwrap()
        .join("../external/eosio.cdt/libraries");
    let dirname = libraries_dir.join(path).canonicalize().unwrap();
    format!("-I{}", dirname.display())
}

fn main() {
    let bindings = bindgen::Builder::default()
        .header("wrapper.hpp")
        .clang_arg(include_library("boost/include").as_str())
        .clang_arg(include_library("libc++/libcxx/include").as_str())
        .clang_arg(include_library("libc/musl/include").as_str())
        .clang_arg(include_library("").as_str())
        .clang_arg("--std=c++14")
        .rustfmt_bindings(true)
        .layout_tests(false)
        .trust_clang_mangling(false)
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");

    // println!("{}", out_path.display());
}
