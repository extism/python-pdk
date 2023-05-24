use extism_pdk::*;
use rustpython::vm as vm;

fn run() -> vm::PyResult<()> {
    vm::Interpreter::without_stdlib(Default::default()).enter(|vm| {
        let scope = vm.new_scope_with_builtins();

        let code_obj = vm
            .compile(
                r#"1"#,
                vm::compiler::Mode::Exec,
                "<embedded>".to_owned(),
            )
            .map_err(|err| vm.new_syntax_error(&err, Some("ok")))?;

        vm.run_code_obj(code_obj, scope)?;

        Ok(())
    })
}

#[plugin_fn]
pub fn eval(input: String) -> FnResult<()> {
    let _ = run();

    Ok(())
}
