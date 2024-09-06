use crate::*;
use anyhow::Error;

fn contains_import_fn_decorator<R: std::fmt::Debug>(
    f: &rustpython_parser::ast::StmtFunctionDef<R>,
) -> bool {
    for d in f.decorator_list.iter() {
        println!("{:?}", d);
    }

    return false;
}

fn collect<R: std::fmt::Debug>(
    stmt: rustpython_parser::ast::Stmt<R>,
    exports: &mut Vec<String>,
    imports: &mut Vec<Import>,
) {
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
        contains_import_fn_decorator(f);
    }
}

pub(crate) fn find_imports_and_exports(data: String) -> Result<(Vec<Import>, Vec<String>), Error> {
    let parsed = rustpython_parser::parse(&data, rustpython_parser::Mode::Module, "<source>")?
        .expect_module();

    let mut exports = vec![];
    let mut imports = vec![];
    for stmt in parsed.body {
        collect(stmt, &mut exports, &mut imports);
    }
    Ok((imports, exports))
}
