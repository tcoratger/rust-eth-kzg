use std::{env, path::PathBuf};

/// Path to the java directory that we will use to generate the java bindings from
///
/// Relative to the bindings folder.
const PATH_TO_JAVA_BINDINGS_FILE: &str = "java/java_code/src/main/java/ethereum/cryptography";

// These are the files needed to pass to the `javac` command to generate the header file
const INPUT_FILES: [&str; 3] = ["LibEthKZG.java", "CellsAndProofs.java", "Cells.java"];

fn main() {
    let path_to_bindings_dir = path_to_bindings_folder();
    let path_to_java_bindings_file = path_to_bindings_dir.join(PATH_TO_JAVA_BINDINGS_FILE);

    println!(
        "cargo:rerun-if-changed={}",
        path_to_java_bindings_file.as_os_str().to_str().unwrap()
    );

    // Generate the header file
    let mut command = std::process::Command::new("javac");
    command.arg("-h").arg(".");
    for file in &INPUT_FILES {
        command.arg(path_to_java_bindings_file.join(file));
    }
    let output = command.output().expect("Unable to execute command");

    if !output.status.success() {
        let output = std::str::from_utf8(&output.stderr).expect("msg should be utf8");
        panic!("{}", output)
    }
}

fn path_to_bindings_folder() -> PathBuf {
    let crate_dir = env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR env not set");
    let crate_dir = PathBuf::from(crate_dir);
    // Go up two directories to be at bindings parent directory
    let parent = crate_dir
        .parent()
        .expect("No parent directory found")
        .parent()
        .expect("No parent directory found")
        .to_path_buf();
    parent
}
