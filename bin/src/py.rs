use crate::*;
use anyhow::Error;

fn get_import<R: std::fmt::Debug>(
    f: &rustpython_parser::ast::StmtFunctionDef<R>,
    call: &rustpython_parser::ast::ExprCall<R>,
) -> Result<Import, Error> {
    // println!("{:?} {:?}", f, call);
    let mut module = None;
    let mut func = None;

    let n_args = f.args.args.len();
    let has_return = f.returns.is_some();

    if let Some(module_name) = call.args[0].as_constant_expr() {
        if let Some(module_name) = module_name.value.as_str() {
            module = Some(module_name.to_string());
        }
    }

    if let Some(func_name) = call.args[1].as_constant_expr() {
        if let Some(func_name) = func_name.value.as_str() {
            func = Some(func_name.to_string());
        }
    }

    // println!("IMPORT {:?}::{:?}: {n_args} -> {has_return}", module, func);
    match (module, func) {
        (Some(module), Some(func)) => Ok(Import {
            module,
            name: func,
            params: vec![wagen::ValType::I64; n_args],
            results: if has_return {
                vec![wagen::ValType::I64]
            } else {
                vec![]
            },
        }),
        _ => {
            anyhow::bail!("Invalid import, import_fn must include a module name and function name")
        }
    }
}

fn get_export<R: std::fmt::Debug>(
    f: &rustpython_parser::ast::StmtFunctionDef<R>,
    is_plugin_fn: bool,
) -> Result<Export, Error> {
    let func = f.name.to_string();

    let n_args = f.args.args.len();
    let has_return = f.returns.is_some();

    // if is_plugin_fn && n_args > 0 {
    //     anyhow::bail!(
    //         "plugin_fn expects a function with no arguments, {func} should have no arguments"
    //     );
    // }
    Ok(Export {
        name: func,
        is_plugin_fn,
        params: vec![wagen::ValType::I64; 0],
        results: if is_plugin_fn {
            vec![wagen::ValType::I32]
        } else if has_return {
            vec![wagen::ValType::I64]
        } else {
            vec![]
        },
    })
}

fn get_import_fn_decorator<R: std::fmt::Debug>(
    f: &rustpython_parser::ast::StmtFunctionDef<R>,
) -> Result<Option<Import>, Error> {
    for d in f.decorator_list.iter() {
        if let Some(call) = d.as_call_expr() {
            if let Some(name) = call.func.as_attribute_expr() {
                if let Some(n) = name.value.as_name_expr() {
                    if n.id.as_str() == "import_fn"
                        || n.id.as_str() == "extism" && name.attr.as_str() == "import_fn"
                    {
                        return get_import(f, call).map(Some);
                    }
                }
            }
        }
    }

    Ok(None)
}

fn get_export_decorator<R: std::fmt::Debug>(
    f: &rustpython_parser::ast::StmtFunctionDef<R>,
) -> Result<Option<Export>, Error> {
    for d in f.decorator_list.iter() {
        if let Some(call) = d.as_call_expr() {
            if let Some(name) = call.func.as_attribute_expr() {
                if let Some(n) = name.value.as_name_expr() {
                    if n.id.as_str() == "plugin_fn"
                        || n.id.as_str() == "extism" && name.attr.as_str() == "plugin_fn"
                    {
                        anyhow::bail!("extism.plugin_fn takes no arguments");
                    } else if n.id.as_str() == "shared_fn"
                        || n.id.as_str() == "extism" && name.attr.as_str() == "shared_fn"
                    {
                        anyhow::bail!("extism.shared_fn takes no arguments");
                    }
                }
            }
        } else if let Some(attr) = d.as_attribute_expr() {
            if let Some(n) = attr.value.as_name_expr() {
                if n.id.as_str() == "plugin_fn"
                    || n.id.as_str() == "extism" && attr.attr.as_str() == "plugin_fn"
                {
                    return get_export(f, true).map(Some);
                } else if n.id.as_str() == "shared_fn"
                    || n.id.as_str() == "extism" && attr.attr.as_str() == "shared_fn"
                {
                    return get_export(f, false).map(Some);
                }
            }
        }
    }

    Ok(None)
}

fn collect<R: std::fmt::Debug>(
    stmt: rustpython_parser::ast::Stmt<R>,
    exports: &mut Vec<Export>,
    imports: &mut Vec<Import>,
) -> Result<(), Error> {
    if let Some(f) = stmt.as_function_def_stmt() {
        if let Some(import) = get_import_fn_decorator(f)? {
            imports.push(import);
        } else if let Some(export) = get_export_decorator(f)? {
            exports.push(export);
        }
    }

    Ok(())
}

pub(crate) fn find_imports_and_exports(data: String) -> Result<(Vec<Import>, Vec<Export>), Error> {
    let parsed = rustpython_parser::parse(&data, rustpython_parser::Mode::Module, "<source>")?
        .expect_module();

    let mut exports = vec![];
    let mut imports = vec![];
    for stmt in parsed.body {
        collect(stmt, &mut exports, &mut imports)?;
    }
    Ok((imports, exports))
}
