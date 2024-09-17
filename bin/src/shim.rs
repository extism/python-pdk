use crate::*;
use wagen::{Instr, ValType};

pub(crate) fn generate(
    exports: &[Export],
    imports: &[Import],
    shim_path: &std::path::Path,
) -> Result<(), Error> {
    let mut module = wagen::Module::new();

    let n_imports = imports.len();
    let import_table = module.tables().push(wagen::TableType {
        element_type: wagen::RefType::FUNCREF,
        minimum: n_imports as u64,
        maximum: None,
        table64: false,
    });

    let __arg_start = module.import("core", "__arg_start", None, [], []);
    let __arg_i32 = module.import("core", "__arg_i32", None, [ValType::I32], []);
    let __arg_i64 = module.import("core", "__arg_i64", None, [ValType::I64], []);
    let __arg_f32 = module.import("core", "__arg_f32", None, [ValType::F32], []);
    let __arg_f64 = module.import("core", "__arg_f64", None, [ValType::F64], []);

    let __invoke = module.import(
        "core",
        "__invoke",
        None,
        [wagen::ValType::I32, wagen::ValType::I32],
        [],
    );

    let __invoke_i64 = module.import(
        "core",
        "__invoke_i64",
        None,
        [wagen::ValType::I32, wagen::ValType::I32],
        [wagen::ValType::I64],
    );

    let __invoke_i32 = module.import(
        "core",
        "__invoke_i32",
        None,
        [wagen::ValType::I32, wagen::ValType::I32],
        [wagen::ValType::I32],
    );

    let mut import_elements = Vec::new();
    for import in imports.iter() {
        let index = module.import(
            &import.module,
            &import.name,
            None,
            import.params.clone(),
            import.results.clone(),
        );
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

    for (index, export) in exports.iter().enumerate() {
        if export.results.len() > 1 {
            anyhow::bail!(
                "Multiple return arguments are not currently supported but used in exported function {}",
                export.name
            );
        }
        let func = module
            .func(
                &export.name,
                export.params.clone(),
                export.results.clone(),
                [],
            )
            .export(&export.name);
        let builder = func.builder();
        builder.push(Instr::Call(__arg_start.index()));
        for (parami, param) in export.params.iter().enumerate() {
            builder.push(Instr::LocalGet(parami as u32));

            match param {
                ValType::I32 => {
                    builder.push(Instr::Call(__arg_i32.index()));
                }
                ValType::I64 => {
                    builder.push(Instr::Call(__arg_i64.index()));
                }
                ValType::F32 => {
                    builder.push(Instr::Call(__arg_f32.index()));
                }
                ValType::F64 => {
                    builder.push(Instr::Call(__arg_f64.index()));
                }
                r => {
                    anyhow::bail!("Unsupported param type: {:?}", r);
                }
            }
        }

        builder.push(Instr::I32Const(index as i32));
        builder.push(Instr::I32Const(!export.is_plugin_fn as i32));
        match export.results.first() {
            None => {
                builder.push(Instr::Call(__invoke.index()));
            }
            Some(ValType::I32) => {
                builder.push(Instr::Call(__invoke_i32.index()));
            }
            Some(ValType::I64) => {
                builder.push(Instr::Call(__invoke_i64.index()));
            }
            // Some(ValType::F32) => {
            //     builder.push(Instr::Call(__invoke_f32.index()));
            // }
            // Some(ValType::F64) => {
            //     builder.push(Instr::Call(__invoke_f64.index()));
            // }
            Some(r) => {
                anyhow::bail!("Unsupported result type: {:?}", r);
            }
        }
    }

    module.validate_save(shim_path)?;
    Ok(())
}
