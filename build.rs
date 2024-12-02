use std::env;
use std::path::PathBuf;
use std::process::Command;

fn main() {
    // Get the path to the tree-sitter crate directory
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let tree_sitter_dir = PathBuf::from(manifest_dir).join("tree-sitter-calculator"); // Replace with your tree-sitter crate name

    // Tell cargo to rerun if any files in the tree-sitter directory change
    println!("cargo:rerun-if-changed={}", tree_sitter_dir.display());

    // Run `cargo build` in the tree-sitter directory
    let status = Command::new("cargo")
        .current_dir(&tree_sitter_dir)
        .arg("build")
        .status()
        .expect("Failed to build tree-sitter grammar");

    if !status.success() {
        panic!("Failed to build tree-sitter grammar");
    }
}
