use std::env::var;
use std::process::Command;
use std::io::{self, Write};

fn main() {
    let manifest_dir = var("CARGO_MANIFEST_DIR").unwrap();
    
    let mut command = Command::new("make");
    command
        .arg("-C")
        .arg(format!("{}/../../../usr/mkfs", manifest_dir))
        .arg("build/libfs.a");
    let output = command
        .output()
        .unwrap();

    assert!(output.status.success(), "Command failed:\n{:?}\n{}", command, String::from_utf8_lossy(&output.stderr));

    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed={}/../../../usr/", manifest_dir);
    println!("cargo:rustc-link-search=native={}/../../../usr/mkfs/build", manifest_dir);
    println!("cargo:rustc-link-lib=static=fs");
}