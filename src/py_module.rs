use pyo3::{types::PyModule, PyResult, Python};

#[pyo3::pyfunction]
pub fn input() -> PyResult<String> {
    let input = extism_pdk::input::<String>().unwrap();
    Ok(input)
}

#[pyo3::pyfunction]
pub fn output(result: String) -> PyResult<()> {
    extism_pdk::output(result).unwrap();
    Ok(())
}

#[pyo3::pymodule]
#[pyo3(name = "extism")]
pub fn make_extism_module(_py: Python<'_>, module: &PyModule) -> PyResult<()> {
    module.add_function(pyo3::wrap_pyfunction!(input, module)?)?;
    module.add_function(pyo3::wrap_pyfunction!(output, module)?)?;
    Ok(())
}
