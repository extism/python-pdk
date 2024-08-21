use pyo3::types::{PyModule, PyTuple};
use pyo3::{prelude::*, IntoPy, Py, PyAny, PyResult, Python};

pub mod py_module;
use py_module::make_extism_module;

pub fn call_function<T: IntoPy<Py<PyTuple>>>(
    function_name: &str,
    function_code: &str,
    args: T,
) -> PyResult<()> {
    pyo3::append_to_inittab!(make_extism_module);

    pyo3::prepare_freethreaded_python();

    Python::with_gil(|py| -> PyResult<()> {
        let fun: Py<PyAny> = PyModule::from_code_bound(py, function_code, "", "")?
            .getattr(function_name)?
            .into();

        fun.call1(py, args)?;
        Ok(())
    })
}
