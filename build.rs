fn main() {
    let mut pkg_config = pkg_config::Config::new();
    let pkg_config = pkg_config.atleast_version("2.33.2");
    let libblkid = pkg_config
        .cargo_metadata(false)
        .probe("blkid")
        .expect("Failed to find libblkid?");
    for arg in libblkid.libs.iter() {
        println!("cargo:rustc-link-arg=-l{arg}");
    }
}
