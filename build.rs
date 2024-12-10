use std::env;
use std::path::Path;

fn main() {
    println!("cargo:rerun-if-env-changed=Z3_BASE_PATH");

    // you can put this into `.cargo/config.toml`
    let z3_base_path = Path::new(env!("Z3_BASE_PATH"));

    let z3_include_path = z3_base_path.join("include");
    let z3_library_path = z3_base_path.join("bin");

    // Inform Cargo about the include path
    println!("cargo:include={}", z3_include_path.display());

    // Inform Cargo about the library path
    println!("cargo:rustc-link-search=native={}", z3_library_path.display());

    // Link to the Z3 library
    println!("cargo:rustc-link-lib=dylib=z3");

    // Optionally pass extra arguments to bindgen, if needed
    println!("cargo:rerun-if-env-changed=BINDGEN_EXTRA_CLANG_ARGS");
    env::set_var("BINDGEN_EXTRA_CLANG_ARGS", format!("-I{}", z3_include_path.display()));
}
