fn main() {
    println!("cargo::rerun-if-changed=src/invoke.py");
    println!("cargo::rerun-if-changed=../lib/target/wasm32-wasi/release/core.wasm");

    let out = std::path::PathBuf::from(std::env::var("OUT_DIR").unwrap()).join("core.wasm");
    std::fs::copy("../lib/target/wasm32-wasi/release/core.wasm", out).unwrap();
}
