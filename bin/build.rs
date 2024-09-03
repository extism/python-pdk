fn main() {
    std::fs::copy(
        "../lib/target/wasm32-wasi/release/core.wasm",
        std::path::PathBuf::from(std::env::var("OUT_DIR").unwrap()).join("core.wasm"),
    )
    .unwrap();
}
