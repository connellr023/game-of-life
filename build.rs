use cc::Build;

fn main() {
    Build::new()
        .cpp(true)
        .file("cxx/lib.cpp")
        .compile("lib");

    println!("cargo:rustc-link-search=native={}", std::env::var("OUT_DIR").unwrap());
    println!("cargo:rustc-link-lib=static=lib");
}