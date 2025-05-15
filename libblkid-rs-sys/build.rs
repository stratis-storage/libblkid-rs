use bindgen::Builder;

#[cfg(any(feature = "bindgen_stable_182_rust_target", feature="bindgen_stable_177_rust_target", feature = "bindgen_nightly_rust_target"))]
use bindgen::RustTarget;

use std::{env, path::PathBuf};

fn main() {
    let mut pkg_config = pkg_config::Config::new();
    let pkg_config = pkg_config.atleast_version("2.33.2");
    #[cfg(feature = "static")]
    {
        pkg_config.statik(true);
    }
    let libblkid = pkg_config.probe("blkid").expect("Failed to find libblkid?");

    let builder = Builder::default()
        .clang_args(
            libblkid
                .include_paths
                .iter()
                .map(|include| format!("-I{}", include.display())),
        )
        .header("header.h")
        .size_t_is_usize(true);

    #[cfg(feature = "bindgen_lowest_rust_target")]
    let builder = builder.rust_target(env!("CARGO_PKG_RUST_VERSION").parse().expect("valid"));

    #[cfg(feature = "bindgen_stable_182_rust_target")]
    let builder = builder.rust_target(match RustTarget::stable(82,0) {
        Err(_) => unreachable!("valid rust target"),
        Ok(t) => t,
    });

    #[cfg(feature = "bindgen_stable_177_rust_target")]
    let builder = builder.rust_target(match RustTarget::stable(77,0) {
        Err(_) => unreachable!("valid rust target"),
        Ok(t) => t,
    });

    #[cfg(feature = "bindgen_nightly_rust_target")]
    let builder = builder.rust_target(match RustTarget::nightly() {
        Err(_) => unreachable!("valid rust target"),
        Ok(t) => t,
    });

    let bindings = builder
        .generate()
        .expect("Unable to generate bindings");


    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings");
}
