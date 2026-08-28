use std::path::Path;

fn main() {
    let root = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    let vendor = Path::new(&root).join("vendor");

    println!("cargo:rerun-if-changed=vendor/vmaware.hpp");
    println!("cargo:rerun-if-changed=src/ffi.rs");
    println!("cargo:rerun-if-changed=src/bridge.cpp");
    println!("cargo:rerun-if-changed=bridge.hpp");

    let target_os = std::env::var("CARGO_CFG_TARGET_OS").unwrap();

    cxx_build::bridge("src/ffi.rs")
        .file("src/bridge.cpp")
        .include(&vendor)
        .include(".")
        .flag_if_supported("/std:c++20")
        .flag_if_supported("/EHsc")
        .flag_if_supported("-std=c++20")
        .compile("vmaware_bridge");

    if target_os == "windows" {
        println!("cargo:rustc-link-lib=advapi32");
        println!("cargo:rustc-link-lib=gdi32");
        println!("cargo:rustc-link-lib=user32");
        println!("cargo:rustc-link-lib=setupapi");
        println!("cargo:rustc-link-lib=dxva2");
        println!("cargo:rustc-link-lib=ole32");
        println!("cargo:rustc-link-lib=oleaut32");
        println!("cargo:rustc-link-lib=wbemuuid");
    }
}
