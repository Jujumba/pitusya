use std::process::Command;

fn main() {
    let llvm_flags_process = Command::new("llvm-config")
        .args(["--cflags", "--ldflags", "--system-libs", "--libs", "core"])
        .output()
        .expect("Build error: LLVM is not installed!");
    let llvm_flags = String::from_utf8(llvm_flags_process.stdout).unwrap();

    println!("cargo:rerun-if-changed=src/c/pitusya_llvm_wrapper.c");
    println!("cargo:rustc-link-lib=LLVM-16");
    cc::Build::new()
        .compiler("clang")
        .flag("-c")
        .flag(&llvm_flags)
        .file("src/c/pitusya_llvm_wrapper.c")
        .compile("libpitusya_llvm_wrapper");
    println!("cargo:rustc-link-lib=libpitusya_llvm_wrapper");
}