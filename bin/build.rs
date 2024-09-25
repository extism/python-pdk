fn main() {
    println!("cargo::rerun-if-changed=src/invoke.py");

    let out = std::path::PathBuf::from(std::env::var("OUT_DIR").unwrap()).join("core.wasm");
    if let Ok(path) = std::env::var("EXTISM_ENGINE_PATH") {
        println!("cargo::rerun-if-changed={path}");
        std::fs::copy(path, out).unwrap();
    } else {
        println!("cargo::rerun-if-changed=../lib/target/wasm32-wasi/release/core.wasm");
        std::fs::copy("../lib/target/wasm32-wasi/release/core.wasm", out).unwrap();
    }
}
