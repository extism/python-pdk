use anyhow::Error;

fn collect<R>(stmt: rustpython_parser::ast::Stmt<R>, exports: &mut Vec<String>) {
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

pub(crate) fn find_exports(data: String) -> Result<Vec<String>, Error> {
    let parsed = rustpython_parser::parse(&data, rustpython_parser::Mode::Module, "<source>")?
        .expect_module();

    let mut exports = vec![];
    for stmt in parsed.body {
        collect(stmt, &mut exports);
    }
    Ok(exports)
}
