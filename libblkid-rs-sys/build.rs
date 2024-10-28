use bindgen::Builder;

use cfg_if::cfg_if;
use std::{env, path::PathBuf};

cfg_if! {
    if #[cfg(feature = "pkg-config")] {
        fn setup_bindings_builder() -> Builder {
            let mut pkg_config = pkg_config::Config::new();
            let pkg_config = pkg_config.atleast_version("2.33.2");
            #[cfg(feature = "static")]
            {
                pkg_config.statik(true);
            }
            let libblkid = pkg_config.probe("blkid").expect("Failed to find libblkid?");

            Builder::default().clang_args(libblkid
                .include_paths
                .iter()
                .map(|include|format!("-I{}", include.display())))
        }
    }else {
        fn setup_bindings_builder() -> Builder {
            println!("cargo:rustc-link-lib=blkid");
            Builder::default()
        }
    }
}

fn main() {
    let bindings = setup_bindings_builder()
        .header("header.h")
        .size_t_is_usize(true)
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings");
}
