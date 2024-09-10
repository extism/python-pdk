mod opt;
mod options;
mod py;
mod shim;

use anyhow::{bail, Error};
use log::LevelFilter;
use options::Options;
use structopt::StructOpt;
use tempfile::TempDir;

use std::borrow::Cow;
use std::env;
use std::io::Write;
use std::process::{Command, Stdio};

const CORE: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/core.wasm"));
const INVOKE: &str = include_str!("invoke.py");

#[derive(Debug, Clone)]
struct Import {
    module: String,
    name: String,
    params: Vec<wagen::ValType>,
    results: Vec<wagen::ValType>,
}

#[derive(Debug, Clone)]
struct Export {
    name: String,
    params: Vec<wagen::ValType>,
    results: Vec<wagen::ValType>,
}

fn main() -> Result<(), Error> {
    // Setup logging
    let mut builder = env_logger::Builder::new();
    builder
        .filter(None, LevelFilter::Info)
        .target(env_logger::Target::Stdout)
        .init();

    // Parse CLI arguments
    let opts = Options::from_args();
    let core: Cow<[u8]> = if let Ok(path) = std::env::var("EXTISM_ENGINE_PATH") {
        Cow::Owned(std::fs::read(path)?)
    } else {
        Cow::Borrowed(CORE)
    };

    // Generate core module if `core` flag is set
    if opts.core {
        opt::Optimizer::new(&core)
            .wizen(true)
            .write_optimized_wasm(opts.output)?;
        return Ok(());
    }

    let mut user_code = std::fs::read_to_string(&opts.input_py)?;
    user_code.push_str("\n");
    user_code += INVOKE;

    let tmp_dir = TempDir::new()?;
    let core_path = tmp_dir.path().join("core.wasm");
    let shim_path = tmp_dir.path().join("shim.wasm");

    let self_cmd = env::args().next().expect("Expected a command argument");
    {
        let mut command = Command::new(self_cmd)
            .arg("-c")
            .arg(&opts.input_py)
            .arg("-o")
            .arg(&core_path)
            .stdin(Stdio::piped())
            .spawn()?;
        command
            .stdin
            .take()
            .expect("Expected to get writeable stdin")
            .write_all(user_code.as_bytes())?;
        let status = command.wait()?;
        if !status.success() {
            bail!("Couldn't create wasm from input");
        }
    }

    let (imports, exports) = py::find_imports_and_exports(user_code)?;
    if exports.is_empty() {
        anyhow::bail!(
            "No exports found, use the @extism.plugin_fn decorator to specify exported functions"
        )
    }

    shim::generate(&exports, &imports, &shim_path)?;

    let output = Command::new("wasm-merge")
        .arg("--version")
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status();
    if output.is_err() {
        bail!("Failed to detect wasm-merge. Please install binaryen and make sure wasm-merge is on your path: https://github.com/WebAssembly/binaryen");
    }

    // Merge the shim with the core module
    let status = Command::new("wasm-merge")
        .arg(&core_path)
        .arg("core")
        .arg(&shim_path)
        .arg("shim")
        .arg("-o")
        .arg(&opts.output)
        .arg("--enable-reference-types")
        .arg("--enable-bulk-memory")
        .status()?;
    if !status.success() {
        bail!("wasm-merge failed. Couldn't merge shim");
    }

    opt::optimize_wasm_file(opts.output)?;
    Ok(())
}
