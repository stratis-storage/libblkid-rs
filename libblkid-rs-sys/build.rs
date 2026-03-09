use bindgen::Builder;

use std::{env, path::PathBuf};

fn main() {
    let _ = env::var("LIBBLKID_RS_PKG_CONFIG_PATH").map(|v| env::set_var("PKG_CONFIG_PATH", v));
    let _ = env::var("LIBBLKID_RS_PKG_CONFIG_LIBDIR").map(|v| env::set_var("PKG_CONFIG_LIBDIR", v));
    let _ = env::var("LIBBLKID_RS_PKG_CONFIG_SYSROOT_DIR")
        .map(|v| env::set_var("PKG_CONFIG_SYSROOT_DIR", v));

    let mut pkg_config = pkg_config::Config::new();
    let pkg_config = pkg_config.atleast_version("2.33.2");
    #[cfg(feature = "static")]
    {
        pkg_config.statik(true);
    }
    let libblkid = pkg_config.probe("blkid").expect("Failed to find libblkid?");

    let bindings = Builder::default()
        .rust_target(env!("CARGO_PKG_RUST_VERSION").parse().expect("valid"))
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
