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

fn collect<R: std::fmt::Debug>(
    stmt: rustpython_parser::ast::Stmt<R>,
    exports: &mut Vec<String>,
    imports: &mut Vec<Import>,
) -> Result<(), Error> {
    if let Some(assign) = stmt.as_assign_stmt() {
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
    } else if let Some(f) = stmt.as_function_def_stmt() {
        if let Some(import) = get_import_fn_decorator(f)? {
            imports.push(import);
        }
    }

    Ok(())
}

pub(crate) fn find_imports_and_exports(data: String) -> Result<(Vec<Import>, Vec<String>), Error> {
    let parsed = rustpython_parser::parse(&data, rustpython_parser::Mode::Module, "<source>")?
        .expect_module();

    let mut exports = vec![];
    let mut imports = vec![];
    for stmt in parsed.body {
        collect(stmt, &mut exports, &mut imports)?;
    }
    Ok((imports, exports))
}
