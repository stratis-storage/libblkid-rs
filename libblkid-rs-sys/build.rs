use bindgen;

use std::{env, path::PathBuf};

fn main() {
    println!("cargo:rustc-link-lib=blkid");

    let bindings = bindgen::Builder::default()
        .header("header.h")
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings");
}
