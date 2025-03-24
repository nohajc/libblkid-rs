use bindgen::Builder;

use std::{
    env,
    path::{Path, PathBuf},
};

#[cfg(target_os = "macos")]
fn fix_include(include: &PathBuf) -> &Path {
    if include.ends_with("include/blkid") {
        include.parent().unwrap()
    } else {
        include
    }
}

#[cfg(not(target_os = "macos"))]
fn fix_include(include: &PathBuf) -> &Path {
    include
}

fn main() {
    let mut pkg_config = pkg_config::Config::new();
    let pkg_config = pkg_config.atleast_version("2.33.2");
    #[cfg(feature = "static")]
    {
        pkg_config.statik(true);
    }
    let libblkid = pkg_config.probe("blkid").expect("Failed to find libblkid?");

    let bindings = Builder::default()
        .clang_args(
            libblkid
                .include_paths
                .iter()
                .map(fix_include)
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
