fn main() {
    println!("cargo::rerun-if-env-changed=src/prelude.py");
    println!("cargo::rerun-if-env-changed=../lib/target/wasm32-wasi/release/core.wasm");
    // println!("cargo::rerun-if-env-changed={}", out.display());

    let out = std::path::PathBuf::from(std::env::var("OUT_DIR").unwrap()).join("core.wasm");
    std::process::Command::new("wasm-opt")
        .arg("--disable-reference-types")
        .arg("-O2")
        .arg("../lib/target/wasm32-wasi/release/core.wasm")
        .arg("-o")
        .arg(out)
        .status()
        .unwrap();
}
