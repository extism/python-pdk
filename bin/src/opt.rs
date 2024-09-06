use anyhow::{Error, Result};
use std::{
    path::{Path, PathBuf},
    process::{Command, Stdio},
};
use wizer::Wizer;

pub(crate) struct Optimizer<'a> {
    wizen: bool,
    optimize: bool,
    wasm: &'a [u8],
}

fn find_deps() -> PathBuf {
    let in_repo = PathBuf::from("../lib/target/wasm32-wasi/wasi-deps/usr");
    if in_repo.exists() {
        return in_repo;
    }

    let in_repo_root = PathBuf::from("lib/target/wasm32-wasi/wasi-deps/usr");
    if in_repo_root.exists() {
        return in_repo_root;
    }

    directories::BaseDirs::new()
        .unwrap()
        .data_local_dir()
        .join("extism-py")
}

impl<'a> Optimizer<'a> {
    pub fn new(wasm: &'a [u8]) -> Self {
        Self {
            wasm,
            optimize: false,
            wizen: false,
        }
    }

    #[allow(unused)]
    pub fn optimize(self, optimize: bool) -> Self {
        Self { optimize, ..self }
    }

    pub fn wizen(self, wizen: bool) -> Self {
        Self { wizen, ..self }
    }

    pub fn write_optimized_wasm(self, dest: impl AsRef<Path>) -> Result<(), Error> {
        if self.wizen {
            let wasm = Wizer::new()
                .allow_wasi(true)?
                .inherit_stdio(true)
                .wasm_bulk_memory(true)
                .map_dir("/usr", find_deps())
                .run(self.wasm)?;
            std::fs::write(&dest, wasm)?;
        } else {
            std::fs::write(&dest, self.wasm)?;
        }

        if self.optimize {
            optimize_wasm_file(dest)?;
        }

        Ok(())
    }
}

pub(crate) fn optimize_wasm_file(dest: impl AsRef<Path>) -> Result<(), Error> {
    let output = Command::new("wasm-opt")
        .arg("--version")
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status();
    if output.is_err() {
        anyhow::bail!("Failed to detect wasm-opt. Please install binaryen and make sure wasm-opt is on your path: https://github.com/WebAssembly/binaryen");
    }
    Command::new("wasm-opt")
        .arg("--enable-reference-types")
        .arg("--enable-bulk-memory")
        .arg("--strip")
        .arg("-O3")
        .arg(dest.as_ref())
        .arg("-o")
        .arg(dest.as_ref())
        .status()?;
    Ok(())
}
