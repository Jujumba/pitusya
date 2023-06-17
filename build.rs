fn main() {
    println!("cargo:rerun-if-changed=c/pitusya_llvm_wrapper.c");
    println!("cargo:rustc-link-lib=LLVM-16");
    cc::Build::new()
        .compiler("clang")
        .flag("-c")
        .file("c/pitusya_llvm_wrapper.c")
        .compile("libpitusya_llvm_wrapper");
    println!("cargo:rustc-link-lib=libpitusya_llvm_wrapper");
}