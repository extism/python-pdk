mod opt;
mod options;
mod py;

use anyhow::{bail, Error};
use log::LevelFilter;
use options::Options;
use structopt::StructOpt;
use tempfile::TempDir;

use std::env;
use std::io::Write;
use std::process::{Command, Stdio};

const CORE: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/core.wasm"));
const PRELUDE: &str = include_str!("prelude.py");

fn generate_shim(exports: &[String], shim_path: &std::path::Path) -> Result<(), Error> {
    let mut module = wagen::Module::new();
    let invoke = module.import(
        "core",
        "__invoke",
        None,
        [wagen::ValType::I32],
        [wagen::ValType::I32],
    );

    let mut elements = vec![];
    for (index, export) in exports.iter().enumerate() {
        let fn_index = module
            .func(&export, [], [wagen::ValType::I32], [])
            .with_builder(|b| {
                b.push(wagen::Instr::I32Const(index as i32));
                b.push(wagen::Instr::Call(invoke.index()));
            })
            .export(export);
        elements.push(fn_index.index);
    }

    module.validate_save(&shim_path)?;
    Ok(())
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

    // Generate core module if `core` flag is set
    if opts.core {
        opt::Optimizer::new(CORE)
            .wizen(true)
            .write_optimized_wasm(opts.output)?;
        return Ok(());
    }

    let mut user_code = std::fs::read_to_string(&opts.input_py)?;
    user_code.push_str("\n");
    user_code += PRELUDE;

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

    let exports = py::find_exports(user_code)?;
    if exports.is_empty() {
        anyhow::bail!("No exports found, use __all__ to specify exported functions")
    }

    generate_shim(&exports, &shim_path)?;

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
        .arg("--disable-reference-types")
        .arg("--enable-bulk-memory")
        .status()?;
    if !status.success() {
        bail!("wasm-merge failed. Couldn't merge shim");
    }

    opt::optimize_wasm_file(opts.output)?;
    Ok(())
}
