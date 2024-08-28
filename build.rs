use cc::Build;

fn main() {
    let mut build = {
        let mut build = Build::new();

        build.cpp(true);
        build
    };

    if cfg!(target_os = "windows") {
        println!("cargo:rustc-link-lib=gdi32");
        println!("cargo:rustc-link-lib=user32");

        build.file("cxx/windows_framebuffer.cpp");
    }
    else {
        panic!("Unsupported OS");
    }

    build.compile("lib");

    println!("cargo:rustc-link-search=native={}", std::env::var("OUT_DIR").unwrap());
    println!("cargo:rustc-link-lib=static=lib");
}