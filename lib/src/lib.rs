use pyo3::types::{PyModule, PyTuple, PyTracebackMethods};
use pyo3::{append_to_inittab, conversion::ToPyObject, prelude::*, Py, PyAny, PyResult, Python};

mod py_module;
use py_module::make_extism_ffi_module;

use std::io::Read;

const PRELUDE: &str = include_str!("prelude.py");

fn convert_arg(py: Python, arg: Arg) -> PyObject {
    match arg {
        Arg::Int(x) => x.to_object(py),
        Arg::Float(f) => f.to_object(py),
    }
}

fn wrap_gil<T, F: FnOnce(Python) -> PyResult<T>>(err: T, f: F) -> T {
    let result = Python::with_gil(|py| {
        f(py).map_err(|err| {
            let tb = err.traceback_bound(py).and_then(|x|{
                if let Ok(x) = x.format() {
                    Some(x)
                } else {
                    None
                }
            });
            let mut s = err.into_value(py).to_string();
            if let Some(tb) = tb {
                s += "\n";
                s += &tb;
            }
            s
        })
    });
    match result {
        Ok(x) => x,
        Err(error) => {
            let mem = extism_pdk::Memory::from_bytes(&error)
                        .expect("Load Python error message into Extism memory");
            unsafe {
                extism_pdk::extism::error_set(mem.offset());
            }
            err
        }
    }
}

#[no_mangle]
pub extern "C" fn __invoke(index: u32, shared: bool) {
    wrap_gil((), |py| {
        let call_args = unsafe { CALL_ARGS.pop() };
        let mut args: Vec<PyObject> = call_args
           .unwrap()
            .into_iter()
            .map(|x| convert_arg(py, x))
            .collect();
        args.insert(0, shared.to_object(py));
        args.insert(0, index.to_object(py));
        let args = PyTuple::new_bound(py, args);
        let m = PyModule::import_bound(py, "extism_plugin")?;
        let fun: Py<PyAny> = m.getattr("__invoke")?.into();
        fun.call1(py, args)?;
        Ok(())
    });
}

#[no_mangle]
pub extern "C" fn __invoke_i32(index: u32, shared: bool) -> i32 {
    wrap_gil(-1, |py| -> PyResult<i32> {
        let call_args = unsafe { CALL_ARGS.pop() };
        let mut args: Vec<PyObject> = call_args
            .unwrap()
            .into_iter()
            .map(|x| convert_arg(py, x))
            .collect();
        args.insert(0, shared.to_object(py));
        args.insert(0, index.to_object(py));
        let args = PyTuple::new_bound(py, args);
        let m = PyModule::import_bound(py, "extism_plugin")?;
        let fun: Py<PyAny> = m.getattr("__invoke")?.into();
        let res = fun.call1(py, args)?;
        if let Ok(res) = res.extract(py) {
            return Ok(res);
        }
        Ok(0)
    })
}

#[no_mangle]
pub extern "C" fn __invoke_i64(index: u32, shared: bool) -> i64 {
    wrap_gil(-1, |py| -> PyResult<i64> {
        let call_args = unsafe { CALL_ARGS.pop() };
        let mut args: Vec<PyObject> = call_args
            .unwrap()
            .into_iter()
            .map(|x| convert_arg(py, x))
            .collect();
        args.insert(0, shared.to_object(py));
        args.insert(0, index.to_object(py));
        let args = PyTuple::new_bound(py, args);
        let m = PyModule::import_bound(py, "extism_plugin")?;
        let fun: Py<PyAny> = m.getattr("__invoke")?.into();
        let res = fun.call1(py, args)?;
        if let Ok(res) = res.extract(py) {
            return Ok(res);
        }
        Ok(0)
    }).into()
}

enum Arg {
    Int(i64),
    Float(f64),
}

static mut CALL_ARGS: Vec<Vec<Arg>> = vec![];

#[no_mangle]
pub extern "C" fn __arg_start() {
    unsafe {
        CALL_ARGS.push(vec![]);
    }
}

#[no_mangle]
pub extern "C" fn __arg_i32(arg: i32) {
    unsafe {
        CALL_ARGS.last_mut().unwrap().push(Arg::Int(arg as i64));
    }
}

#[no_mangle]
pub extern "C" fn __arg_i64(arg: i64) {
    unsafe {
        CALL_ARGS.last_mut().unwrap().push(Arg::Int(arg));
    }
}

#[no_mangle]
pub extern "C" fn __arg_f32(arg: f32) {
    unsafe {
        CALL_ARGS.last_mut().unwrap().push(Arg::Float(arg as f64));
    }
}

#[no_mangle]
pub extern "C" fn __arg_f64(arg: f64) {
    unsafe {
        CALL_ARGS.last_mut().unwrap().push(Arg::Float(arg));
    }
}

#[export_name = "wizer.initialize"]
extern "C" fn init() {
    append_to_inittab!(make_extism_ffi_module);
    pyo3::prepare_freethreaded_python();
    let mut code = String::new();
    std::io::stdin().read_to_string(&mut code).unwrap();
    Python::with_gil(|py| -> PyResult<()> {
        PyModule::from_code_bound(py, PRELUDE, "<source>", "extism")?;
        PyModule::from_code_bound(py, &code, "<source>", "extism_plugin")?;
        Ok(())
    })
    .expect("initialize python code")
}
