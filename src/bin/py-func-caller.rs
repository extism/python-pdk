#![no_main]

use extism_python_pdk::call_function;
use extism_python_pdk::py_module::make_extism_module;
use pyo3::append_to_inittab;

#[no_mangle]
pub fn count_vowels() -> i32 {
    append_to_inittab!(make_extism_module);

    let function_code = include_str!("py-func.py");
    call_function("count_vowels", function_code, ()).unwrap();
    0
}
