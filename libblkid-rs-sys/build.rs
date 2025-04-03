use bindgen::Builder;

use std::{env, path::PathBuf};

fn main() {
    let mut pkg_config = pkg_config::Config::new();
    let pkg_config = pkg_config.atleast_version("2.33.2");
    #[cfg(feature = "static")]
    {
        pkg_config.statik(true);
    }

    let host = env::var("HOST").unwrap();
    let target = env::var("TARGET").unwrap();

    if host != target {
        // cross-compilation
        pkg_config.cargo_metadata(false);
    }

    let libblkid = pkg_config.probe("blkid").expect("Failed to find libblkid?");

    let bindings = Builder::default()
        .clang_args(
            libblkid
                .include_paths
                .iter()
                .map(|include| format!("-I{}", include.display())),
        )
        .header("header.h")
        .size_t_is_usize(true)
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings");
}
