use std::{env, path::PathBuf};

/// The directory where the generated header file will be written.
const DIR_FOR_HEADER: &str = "build";

fn main() {
    // linker flags
    // Link libm on Unix-like systems (needed due to use of num_cpus crate)
    #[cfg(not(target_os = "windows"))]
    println!("cargo:rustc-link-lib=m");

    println!("cargo:rerun-if-changed=src/");
    let crate_dir = env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR env not set");
    let package_name = env::var("CARGO_PKG_NAME").expect("CARGO_PKG_NAME env not set");

    let path_to_crate_dir = PathBuf::from(&crate_dir);

    let output_file = PathBuf::from(&path_to_crate_dir)
        .join(DIR_FOR_HEADER)
        .join(format!("{package_name}.h"))
        .display()
        .to_string();

    cbindgen::Builder::new()
        .with_crate(crate_dir)
        .with_language(cbindgen::Language::C)
        .generate()
        .unwrap()
        .write_to_file(output_file);
}
