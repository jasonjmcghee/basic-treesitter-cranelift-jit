use std::process::Command;

fn main() {
    let src_dir = std::path::Path::new("src");

    println!("cargo:rerun-if-changed=grammar.js");
    println!("cargo:rerun-if-changed=src");

    // Generate the parser using tree-sitter CLI
    let status = Command::new("tree-sitter")
        .arg("generate")
        .status()
        .expect("Failed to run tree-sitter generate. Is tree-sitter CLI installed?");

    if !status.success() {
        panic!("Failed to generate parser");
    }

    let mut c_config = cc::Build::new();
    c_config.std("c11").include(src_dir);

    #[cfg(target_env = "msvc")]
    c_config.flag("-utf-8");

    let parser_path = src_dir.join("parser.c");
    c_config.file(&parser_path);
    println!("cargo:rerun-if-changed={}", parser_path.to_str().unwrap());

    // NOTE: if your language uses an external scanner, uncomment this block:
    /*
    let scanner_path = src_dir.join("scanner.c");
    c_config.file(&scanner_path);
    println!("cargo:rerun-if-changed={}", scanner_path.to_str().unwrap());
    */

    c_config.compile("tree-sitter-calculator");
}
