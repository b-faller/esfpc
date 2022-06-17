use std::path::PathBuf;

const OUTPUT_DIR: &str = "target/i686-pc-windows-msvc/release";

/// Copies the rules directory into the release folder for easier testing with EuroScope.
fn copy_rules_dir() -> std::io::Result<()> {
    let out_rules_dir = PathBuf::from(OUTPUT_DIR).join("rules");
    if !out_rules_dir.exists() {
        std::fs::create_dir_all(&out_rules_dir)?;
    }

    for entry in std::fs::read_dir("rules")? {
        let entry = entry?;
        std::fs::copy(entry.path(), out_rules_dir.join(entry.file_name()))?;
    }

    Ok(())
}

fn main() {
    let _build = cxx_build::bridge("src/lib.rs")
        .flag("/std:c++20")
        .flag("/permissive-")
        .flag("/W4")
        .file("cxx/main.cpp")
        .file("cxx/util.cpp")
        .compile("esfpc");

    println!("cargo:rustc-link-search=lib");
    println!("cargo:rustc-link-lib=static=EuroScopePlugInDll");

    copy_rules_dir().unwrap();
}
