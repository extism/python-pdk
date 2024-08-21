use pyo3::{exceptions::PyException, prelude::*, types::PyModule, PyErr, PyResult};

fn error(x: extism_pdk::Error) -> PyErr {
    PyException::new_err(format!("{:?}", x))
}

#[pyo3::pyfunction]
pub fn input() -> PyResult<String> {
    let input = extism_pdk::input::<String>().map_err(error)?;
    Ok(input)
}

#[pyo3::pyfunction]
pub fn output(result: String) -> PyResult<()> {
    extism_pdk::output(result).map_err(error)?;
    Ok(())
}

#[pyo3::pyfunction]
pub fn config_get(key: String) -> PyResult<Option<String>> {
    let r = extism_pdk::config::get(key).map_err(error)?;
    Ok(r)
}

#[pyo3::pyfunction]
pub fn var_get(key: String) -> PyResult<Option<String>> {
    let r = extism_pdk::var::get(key).map_err(error)?;
    Ok(r)
}

#[pyo3::pyfunction]
pub fn var_set(key: String, value: Vec<u8>) -> PyResult<()> {
    extism_pdk::var::set(key, value).map_err(error)?;
    Ok(())
}

#[pyo3::pymodule]
#[pyo3(name = "extism")]
pub fn make_extism_module(module: &Bound<'_, PyModule>) -> PyResult<()> {
    module.add_function(pyo3::wrap_pyfunction!(input, module)?)?;
    module.add_function(pyo3::wrap_pyfunction!(output, module)?)?;
    module.add_function(pyo3::wrap_pyfunction!(config_get, module)?)?;
    module.add_function(pyo3::wrap_pyfunction!(var_get, module)?)?;
    module.add_function(pyo3::wrap_pyfunction!(var_set, module)?)?;
    Ok(())
}
