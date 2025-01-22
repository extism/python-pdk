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
    debug: bool,
}

fn find_deps() -> PathBuf {
    if let Ok(path) = std::env::var("EXTISM_PYTHON_WASI_DEPS_DIR") {
        return PathBuf::from(path);
    }

    let in_repo = PathBuf::from("../lib/target/wasm32-wasi/wasi-deps");
    if in_repo.exists() {
        return in_repo;
    }

    let in_repo_root = PathBuf::from("lib/target/wasm32-wasi/wasi-deps");
    if in_repo_root.exists() {
        return in_repo_root;
    }

    let usr_local_share = PathBuf::from("/usr/local/share/extism-py");
    if usr_local_share.exists() {
        return usr_local_share;
    }

    let usr_share = PathBuf::from("/usr/share/extism-py");
    if usr_share.exists() {
        return usr_share;
    }

    directories::BaseDirs::new()
        .unwrap()
        .data_dir()
        .join("extism-py")
}

impl<'a> Optimizer<'a> {
    pub fn new(wasm: &'a [u8]) -> Self {
        Self {
            wasm,
            optimize: false,
            wizen: false,
            debug: false,
        }
    }

    #[allow(unused)]
    pub fn optimize(self, optimize: bool) -> Self {
        Self { optimize, ..self }
    }

    #[allow(unused)]
    pub fn debug(self, debug: bool) -> Self {
        Self { debug, ..self }
    }

    pub fn wizen(self, wizen: bool) -> Self {
        Self { wizen, ..self }
    }

    #[cfg(target_os = "windows")]
    fn convert_windows_paths(&self, paths: Vec<(String, PathBuf)>) -> Vec<(String, PathBuf)> {
        use std::path::Component;
        let mut ret = vec![];
        for (_, path) in paths {
            let new_path = 
                path.components()
                    .filter_map(|comp| match comp {
                        Component::Normal(part) => Some(part.to_string_lossy().to_string()),
                        _ => None, // Skip root, prefix, or other non-normal components
                    })
                    .collect::<Vec<_>>()
                    .join("/");
            let normalized = format!("/{}", new_path);
            ret.push((normalized, path));
        }
        ret
    }

    pub fn write_optimized_wasm(self, dest: impl AsRef<Path>) -> Result<(), Error> {
        let python_path = std::env::var("PYTHONPATH").unwrap_or_else(|_| String::from("."));
        let split_paths = std::env::split_paths(&python_path);
        let paths: Vec<(String, PathBuf)> = split_paths.map(|p| (p.to_string_lossy().to_string(), p)).collect();
        
        #[cfg(target_os = "windows")]
        let paths = self.convert_windows_paths(paths);
        #[cfg(target_os = "windows")]
        std::env::set_var("PYTHONPATH", paths.iter().map(|p| p.0.clone()).collect::<Vec<_>>().join(":"));

        // Ensure compatibility with old releases
        let mut deps = find_deps().join("usr");
        if !deps.exists() {
            let parent = deps.parent().unwrap();
            if parent.join("local").exists() {
                deps = parent.to_path_buf();
            } else {
                anyhow::bail!("wasi-deps path doesn't exist: {}", deps.display());
            }
        }

        if self.wizen {
            let mut w = Wizer::new();
            w.allow_wasi(true)?
                .inherit_stdio(true)
                .inherit_env(true)
                .wasm_bulk_memory(true)
                .map_dir("/usr", deps);
            for (mapped, path) in paths {
                if !path.exists() {
                    continue;
                }
                w.map_dir(mapped, path);
            }
            let wasm = w.run(self.wasm)?;
            std::fs::write(&dest, wasm)?;
        } else {
            std::fs::write(&dest, self.wasm)?;
        }

        if self.optimize {
            optimize_wasm_file(dest, self.debug)?;
        }

        Ok(())
    }
}

pub(crate) fn optimize_wasm_file(dest: impl AsRef<Path>, debug: bool) -> Result<(), Error> {
    let output = Command::new("wasm-opt")
        .arg("--version")
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status();
    if output.is_err() {
        anyhow::bail!("Failed to detect wasm-opt. Please install binaryen and make sure wasm-opt is on your path: https://github.com/WebAssembly/binaryen");
    }
    let mut cmd = Command::new("wasm-opt");
    cmd.arg("--enable-reference-types")
        .arg("--enable-bulk-memory")
        .arg("-O2");
    if debug {
        cmd.arg("-g");
    } else {
        cmd.arg("--strip");
    }
    cmd.arg(dest.as_ref())
        .arg("-o")
        .arg(dest.as_ref())
        .status()?;
    Ok(())
}
