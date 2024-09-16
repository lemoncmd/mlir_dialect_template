use std::{env, path::Path};

fn main() {
    let dst = cmake::build("mlir");
    println!("cargo:rustc-link-search=native={}", dst.display());
    println!("cargo:rustc-link-lib=static=SampleCAPI");
    println!("cargo:rustc-link-lib=static=MLIRSample");
    println!("cargo:rerun-if-changed=mlir");
    println!("cargo:root={}", env::var("OUT_DIR").unwrap());

    bindgen::builder()
        .header("mlir/include/sample-c/Dialects.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .generate()
        .unwrap()
        .write_to_file(Path::new(&env::var("OUT_DIR").unwrap()).join("bindings.rs"))
        .unwrap();
}
