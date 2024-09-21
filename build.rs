use std::{env, path::Path};

fn main() {
    let dst = cmake::Config::new("mlir")
        // .register_dep("MLIRFoo") // when you want to register dependency
        .build();
    println!("cargo:rustc-link-search=native={}", dst.display());
    println!("cargo:rustc-link-lib=static=SampleCAPI");
    println!("cargo:rustc-link-lib=static=MLIRSample");
    println!("cargo:rerun-if-changed=mlir");
    println!("cargo:root={}", env::var("OUT_DIR").unwrap());

    let header = "mlir/include/sample-c/Dialects.h";

    bindgen::builder()
        .header(header)
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .allowlist_recursively(false)
        .allowlist_file(header)
        .generate()
        .unwrap()
        .write_to_file(Path::new(&env::var("OUT_DIR").unwrap()).join("bindings.rs"))
        .unwrap();
}
