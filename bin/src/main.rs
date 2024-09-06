mod opt;
mod options;
mod py;

use anyhow::{bail, Error};
use log::LevelFilter;
use options::Options;
use structopt::StructOpt;
use tempfile::TempDir;
use wagen::{Instr, ValType};

use std::env;
use std::io::Write;
use std::process::{Command, Stdio};

const CORE: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/core.wasm"));
const INVOKE: &str = include_str!("invoke.py");

struct Import {
    module: String,
    name: String,
    params: Vec<wagen::ValType>,
    results: Vec<wagen::ValType>,
}

fn generate_shim(
    exports: &[String],
    imports: &[Import],
    shim_path: &std::path::Path,
) -> Result<(), Error> {
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

    let n_imports = imports.len();
    let import_table = module.tables().push(wagen::TableType {
        element_type: wagen::RefType::FUNCREF,
        minimum: n_imports as u64,
        maximum: None,
        table64: false,
    });

    let mut import_elements = Vec::new();
    let mut import_items = vec![];
    for import in imports.iter() {
        let index = module.import(
            &import.module,
            &import.name,
            None,
            import.params.clone(),
            import.results.clone(),
        );
        import_items.push((format!("{}_{}", import.module, import.name), index));
    }
    import_items.sort_by_key(|x| x.0.to_string());

    for (_f, index) in import_items {
        import_elements.push(index.index());
    }

    for p in 0..=5 {
        for q in 0..=1 {
            let indirect_type = module
                .types()
                .push(|t| t.function(vec![ValType::I64; p], vec![ValType::I64; q]));
            let name = format!("__invokeHostFunc_{p}_{q}");
            let mut params = vec![ValType::I32];
            for _ in 0..p {
                params.push(ValType::I64);
            }
            let invoke_host = module
                .func(&name, params, vec![ValType::I64; q], [])
                .export(&name);
            let builder = invoke_host.builder();
            for i in 1..=p {
                builder.push(Instr::LocalGet(i as u32));
            }
            builder.push(Instr::LocalGet(0));
            builder.push(Instr::CallIndirect {
                ty: indirect_type,
                table: import_table,
            });
        }
    }
    module.active_element(
        Some(import_table),
        wagen::Elements::Functions(&import_elements),
    );

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
    user_code += INVOKE;

    // let tmp_dir = TempDir::new()?;
    // let core_path = tmp_dir.path().join("core.wasm");
    // let shim_path = tmp_dir.path().join("shim.wasm");
    let tmp_dir = std::path::PathBuf::from("tmp");
    let core_path = tmp_dir.join("core.wasm");
    let shim_path = tmp_dir.join("shim.wasm");

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

    generate_shim(&exports, &[], &shim_path)?;

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
