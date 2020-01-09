use bindgen;

use std::path::PathBuf;

fn main() {
    println!("cargo:rustc-link-lib=blkid");

    let bindings = bindgen::Builder::default()
        .header("header.h")
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from("src/bindings.rs");
    bindings
        .write_to_file(out_path)
        .expect("Couldn't write bindings");
}
