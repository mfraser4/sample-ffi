extern crate bindgen;

use std::env;
use std::path::PathBuf;

fn main() {
    let prefix = format!("{}/opt", env::var("HOME").expect("HOME should be available"));
    let includedir = format!("{}/include", &prefix);
    let libdir = format!("{}/lib", &prefix);

    // Specify the foo library and rebuild if the header changes
    println!("cargo:rustc-link-lib=foo");
    println!("cargo:rustc-link-search={}", libdir);
    println!("cargo:rerun-if-changed={}/foo.h", &includedir);

    let bindings = bindgen::Builder::default()
        .header(format!("{}/foo.h", &includedir))
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
