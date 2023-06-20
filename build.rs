use dotenv::dotenv;
use std::process::Command;

fn main() {
    dotenv().expect("Missing the .env file in the source root");
    let llvm_version = std::env::var("LLVM_VERSION").expect("Specify the LLVM version to link!");

    println!("cargo:rustc-link-lib=LLVM-{}", llvm_version);
    println!("cargo:rerun-if-changed=src/c/pitusya_llvm_wrapper.c");

    let llvm_flags_process = Command::new("llvm-config")
        .args(["--cflags", "--ldflags", "--system-libs", "--libs", "core"])
        .output()
        .expect("Build error: LLVM is not installed!");
    let llvm_flags = String::from_utf8(llvm_flags_process.stdout).unwrap();

    cc::Build::new()
        .compiler("clang")
        .flag("-c")
        .flag(&llvm_flags)
        .file("src/c/pitusya_llvm_wrapper.c")
        .compile("libpitusya_llvm_wrapper");
    println!("cargo:rustc-link-lib=libpitusya_llvm_wrapper");
}
