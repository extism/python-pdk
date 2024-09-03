use pyo3::types::PyModule;
use pyo3::{append_to_inittab, prelude::*, Py, PyAny, PyResult, Python};

mod py_module;
use py_module::make_extism_module;

use std::io::Read;

#[no_mangle]
pub extern "C" fn __invoke(index: i32) -> i32 {
    append_to_inittab!(make_extism_module);
    pyo3::prepare_freethreaded_python();

    Python::with_gil(|py| -> PyResult<i32> {
        let m = PyModule::import_bound(py, "extism_plugin")?;

        let fun: Py<PyAny> = m.getattr("__invoke")?.into();

        let res = fun.call1(py, (index,))?;
        if let Ok(res) = res.extract(py) {
            return Ok(res);
        }
        Ok(0)
    })
    .unwrap()
}

#[export_name = "wizer.initialize"]
extern "C" fn init() {
    append_to_inittab!(make_extism_module);
    pyo3::prepare_freethreaded_python();
    let mut code = String::new();
    std::io::stdin().read_to_string(&mut code).unwrap();
    Python::with_gil(|py| -> PyResult<()> {
        let m = PyModule::from_code_bound(py, &code, "<source>", "extism_plugin")?;
        m.getattr("__all__")
            .expect("__all__ is required to list exports");
        Ok(())
    })
    .expect("initialize python code")
}
