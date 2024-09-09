use crate::*;
use wagen::{Instr, ValType};

pub(crate) fn generate(
    exports: &[Export],
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
            .func(
                &export.name,
                export.params.clone(),
                export.results.clone(),
                [],
            )
            .with_builder(|b| {
                b.push(wagen::Instr::I32Const(index as i32));
                b.push(wagen::Instr::Call(invoke.index()));
            })
            .export(&export.name);
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
