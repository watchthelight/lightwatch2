//! Build script for LIGHTWATCH

fn main() {
    // Pass target triple to the binary
    println!(
        "cargo:rustc-env=TARGET={}",
        std::env::var("TARGET").unwrap()
    );

    // Rebuild if these change
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=assets/");
}
