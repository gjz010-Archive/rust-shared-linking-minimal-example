fn main() {
    println!("cargo:rerun-if-changed=lib/hello.c");
    println!("cargo:rustc-link-lib=dylib=hello");
    println!("cargo:rustc-link-search=all=lib/");
}