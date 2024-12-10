use std::env;
use std::path::Path;

fn main() {
    println!("cargo:rerun-if-env-changed=Z3_BASE_PATH");
    println!("cargo:rerun-if-env-changed=BINDGEN_EXTRA_CLANG_ARGS");

    // you can put this into `.cargo/config.toml`
    // also set `BINDGEN_EXTRA_CLANG_ARGS` to `-I <Z3_BASE_PATH>/include`
    let z3_base_path = Path::new(env!("Z3_BASE_PATH"));

    let z3_library_path = z3_base_path.join("bin");
    println!(
        "cargo:rustc-link-search=native={}",
        z3_library_path.display()
    );
}
