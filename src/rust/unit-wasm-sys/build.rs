// buildscript for the unit-wasm-sys crate.

use std::env;
use std::path::{Path, PathBuf};

fn main() {
    // Tell rustc where to find the libunit-wasm library.
    let libunit_wasm_dir = "libunit-wasm";

    let dir = env::var("CARGO_MANIFEST_DIR").unwrap();

    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=libunit-wasm/include/unit/unit-wasm.h");

    // The rustc-link-search tells Cargo to pass the `-L` flag to the
    // compiler to add a directory to the library search plugin. The
    // `native` keyword means "only looking for `native libraries` in
    // this directory".
    println!(
        "cargo:rustc-link-search=native={}",
        Path::new(&dir).join(libunit_wasm_dir).display()
    );

    // The rustc-link-lib tells Cargo to link the given library using
    // the compiler's `-l` flag. This is needed to start building our
    // FFIs.
    println!("cargo:rustc-link-lib=static=unit-wasm");

    generate_bindings();
}

fn generate_bindings() {
    let bindings = bindgen::Builder::default()
        // The input header file.
        .header("libunit-wasm/include/unit/unit-wasm.h")
        .allowlist_function("^luw_.*")
        .allowlist_var("^luw_.*")
        .allowlist_type("^luw_.*")
        .default_enum_style(bindgen::EnumVariation::Rust {
            non_exhaustive: false,
        })
        .generate()
        .expect("Unable to generate bindings");

    let out_dir_env = env::var("OUT_DIR")
        .expect("The required environment variable OUT_DIR was not set");
    let out_path = PathBuf::from(out_dir_env);

    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
