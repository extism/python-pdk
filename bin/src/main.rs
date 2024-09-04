mod opt;
mod options;

use anyhow::{bail, Error};
use opt::Optimizer;
use options::Options;
use structopt::StructOpt;
use tempfile::TempDir;

use std::env;
use std::io::Write;
use std::process::{Command, Stdio};

const CORE: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/core.wasm"));
const PRELUDE: &str = include_str!("prelude.py");

fn find_exports(data: String) -> Result<Vec<String>, Error> {
    let parsed = rustpython_parser::parse(&data, rustpython_parser::Mode::Module, "<source>")?
        .expect_module();

    let mut exports = vec![];
    for stmt in parsed.body {
        if let Some(assign) = stmt.assign_stmt() {
            if assign.targets.len() == 1 {
                if let Some(expr) = assign.targets[0].as_name_expr() {
                    if expr.id.as_str() == "__all__" {
                        if let Some(list) = assign.value.as_list_expr() {
                            for item in &list.elts {
                                if let Some(name) = item.as_constant_expr() {
                                    if let Some(name) = name.value.as_str() {
                                        exports.push(name.clone());
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    Ok(exports)
}

fn main() -> Result<(), Error> {
    let opts = Options::from_args();
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
    // let tmp_dir = std::path::PathBuf::from("tmp");
    // let core_path = tmp_dir.as_path().join("core.wasm");
    // let shim_path = tmp_dir.as_path().join("shim.wasm");

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

    let mut dest = wagen::Module::new();

    let exports = find_exports(user_code)?;
    if exports.is_empty() {
        anyhow::bail!("No exports found, use __all__ to specify exported functions")
    }

    let invoke = dest.import(
        "core",
        "__invoke",
        None,
        [wagen::ValType::I32],
        [wagen::ValType::I32],
    );

    let mut elements = vec![];
    for (index, export) in exports.into_iter().enumerate() {
        println!("EXPORT: {}", export);
        let fn_index = dest
            .func(&export, [], [wagen::ValType::I32], [])
            .with_builder(|b| {
                b.push(wagen::Instr::I32Const(index as i32));
                b.push(wagen::Instr::Call(invoke.index()));
            })
            .export(&export);
        elements.push(fn_index.index);
    }

    dest.validate_save(&shim_path)?;

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
