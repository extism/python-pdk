use pyo3::{append_to_inittab, PyResult};

use extism_python_pdk::call_function;
use extism_python_pdk::py_module::make_extism_module;

pub fn main() -> PyResult<()> {
    append_to_inittab!(make_extism_module);

    let function_code = include_str!("py-func.py");
    call_function("my_func", function_code, ())
}
