use std::path::Path;

fn main() {
    let root = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    let vmaware_hpp = Path::new(&root).join("vmaware.hpp");

    if !vmaware_hpp.exists() {
        const URL: &str =
            "https://raw.githubusercontent.com/kernelwernel/VMAware/a4c25ff8854e3833d1fc8491ad8692193b7b7d6c/src/vmaware.hpp";
        eprintln!("cargo:warning=vmaware.hpp not found — downloading from {URL}");

        let status = std::process::Command::new("curl")
            .args([
                "--fail",
                "--silent",
                "--show-error",
                "--location",
                "-o",
                vmaware_hpp.to_str().unwrap(),
                URL,
            ])
            .status()
            .expect("failed to invoke `curl`");

        assert!(
            status.success(),
            "curl exited with {status} while downloading vmaware.hpp from {URL}"
        );
    }

    println!("cargo:rerun-if-changed=vmaware.hpp");

    let target_os = std::env::var("CARGO_CFG_TARGET_OS").unwrap();

    cxx_build::bridge("src/ffi.rs")
        .file("src/bridge.cpp")
        .include(&root)
        .flag_if_supported("/std:c++20")
        .flag_if_supported("-std=c++20")
        .compile("vmaware_bridge");

    match target_os.as_str() {
        "windows" => {
            println!("cargo:rustc-link-lib=advapi32");
            println!("cargo:rustc-link-lib=gdi32");
            println!("cargo:rustc-link-lib=user32");
            println!("cargo:rustc-link-lib=setupapi");
            println!("cargo:rustc-link-lib=dxva2");
            println!("cargo:rustc-link-lib=ole32");
            println!("cargo:rustc-link-lib=oleaut32");
            println!("cargo:rustc-link-lib=wbemuuid");
        }
        _ => {}
    }
}
