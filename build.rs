use std::process::Command;

use dotenv::dotenv;

fn main() {
    dotenv().expect("Missing the .env file in the source root");
    let llvm_version = std::env::var("LLVM_VERSION").expect("Build error: specify the LLVM version to link!");

    println!("cargo:rustc-link-lib=LLVM-{}", llvm_version);
    println!("cargo:rerun-if-changed=src/c/pitusya_llvm_wrapper.c");

    let llvm_flags_process = Command::new("llvm-config")
        .args(["--cflags", "--ldflags", "--system-libs", "--libs", "core"])
        .output()
        .expect("Build error: LLVM is not installed!");
    let llvm_flags = String::from_utf8(llvm_flags_process.stdout).unwrap();

    let llvm_includedir = String::from_utf8(Command::new("llvm-config").args(["--includedir"]).output().expect("Build error: LLVM is not installed!").stdout).unwrap();
    let llvm_includedir = llvm_includedir.trim();

    let mut libpitusya = cc::Build::new();
    libpitusya.compiler("clang");

    let profile = std::env::var("PROFILE").unwrap();
    if profile.as_str() == "debug" {
        libpitusya.flag("-g");
    }

    libpitusya
        .flag("-c")
        .flag(&llvm_flags)
        .include(llvm_includedir)
        .file("src/c/pitusya_llvm_wrapper.c")
        .compile("libpitusya_llvm_wrapper");

    println!("cargo:rustc-link-lib=libpitusya_llvm_wrapper");
}
