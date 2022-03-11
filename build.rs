extern crate bindgen;

use std::env;
use std::path::PathBuf;

#[cfg(target_os = "windows")]
fn main() {
    trait ToStr {
        fn to_str(&self) -> &str;
    }

    impl ToStr for PathBuf {
        fn to_str(&self) -> &str {
            self.as_os_str().to_str().unwrap()
        }
    }

    let bonjour_sdk_dir = PathBuf::from(env::var("BONJOUR_SDK_HOME").unwrap());
    let bonjour_sdk_lib_dir = bonjour_sdk_dir.join(r#"lib\x64"#);
    let bonjour_sdk_header_dir = bonjour_sdk_dir.join("include");

    println!("cargo:rustc-link-search={}", bonjour_sdk_lib_dir.to_str());
    println!("cargo:rustc-link-lib=dnssd");

    bindgen::Builder::default()
        // .clang_arg(format!("--library-directory={}", bonjour_sdk_dir.to_str()))
        .clang_arg(format!(
            "--include-directory={}",
            bonjour_sdk_header_dir.to_str()
        ))
        .header("wrapper.h")
        .ctypes_prefix("::libc")
        .blocklist_type("_?P?IMAGE_TLS_DIRECTORY.*")
        .generate()
        .expect("failed to generate bindings")
        .write_to_file(PathBuf::from(env::var("OUT_DIR").unwrap()).join("bindings.rs"))
        .expect("failed to write bindings to file");
}

#[cfg(not(target_os = "windows"))]
fn main() {
    bindgen::Builder::default()
        .header("wrapper.h")
        .ctypes_prefix("::libc")
        .generate()
        .expect("failed to generate bindings")
        .write_to_file(PathBuf::from(env::var("OUT_DIR").unwrap()).join("bindings.rs"))
        .expect("failed to write bindings to file");
}
