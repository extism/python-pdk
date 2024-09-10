use pyo3::{
    exceptions::PyException,
    prelude::*,
    types::{PyBytes, PyModule, PyTuple},
    PyErr, PyResult,
};

use std::collections::BTreeMap;

fn error(x: extism_pdk::Error) -> PyErr {
    PyException::new_err(format!("{:?}", x))
}

#[pyo3::pyfunction]
pub fn input_bytes(py: Python<'_>) -> PyResult<Bound<'_, PyBytes>> {
    let input = extism_pdk::input::<Vec<u8>>().map_err(error)?;
    Ok(PyBytes::new_bound(py, &input))
}

#[pyo3::pyfunction]
pub fn output_bytes(result: &[u8]) -> PyResult<()> {
    extism_pdk::output(result).map_err(error)?;
    Ok(())
}

#[pyo3::pyfunction]
pub fn input_str() -> PyResult<String> {
    let input = extism_pdk::input::<String>().map_err(error)?;
    Ok(input)
}

#[pyo3::pyfunction]
pub fn output_str(result: &str) -> PyResult<()> {
    extism_pdk::output(result).map_err(error)?;
    Ok(())
}

#[pyo3::pyfunction]
pub fn set_error(msg: &str) -> PyResult<()> {
    let mem = extism_pdk::Memory::from_bytes(&msg).map_err(error)?;
    unsafe {
        extism_pdk::extism::error_set(mem.offset());
    }
    Ok(())
}

#[pyo3::pyfunction]
pub fn config_get(key: &str) -> PyResult<Option<String>> {
    let r = extism_pdk::config::get(key).map_err(error)?;
    Ok(r)
}

#[pyo3::pyfunction]
pub fn var_get<'a>(py: Python<'a>, key: &'a str) -> PyResult<Option<Bound<'a, PyBytes>>> {
    let r: Option<Vec<u8>> = extism_pdk::var::get(key).map_err(error)?;
    if let Some(r) = r {
        Ok(Some(PyBytes::new_bound(py, &r)))
    } else {
        Ok(None)
    }
}

#[pyo3::pyfunction]
pub fn var_set(key: String, value: &[u8]) -> PyResult<()> {
    extism_pdk::var::set(key, value).map_err(error)?;
    Ok(())
}

#[pyo3::pyclass(eq, eq_int)]
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum LogLevel {
    Error,
    Debug,
    Warn,
    Info,
}

#[pyo3::pyfunction]
pub fn log(level: LogLevel, msg: &str) -> PyResult<()> {
    match level {
        LogLevel::Error => extism_pdk::log!(extism_pdk::LogLevel::Error, "{}", msg),
        LogLevel::Debug => extism_pdk::log!(extism_pdk::LogLevel::Debug, "{}", msg),
        LogLevel::Warn => extism_pdk::log!(extism_pdk::LogLevel::Warn, "{}", msg),
        LogLevel::Info => extism_pdk::log!(extism_pdk::LogLevel::Info, "{}", msg),
    }

    Ok(())
}

#[pyo3::pyclass(eq)]
#[derive(Debug, PartialEq, Clone)]
pub struct HttpRequest {
    #[pyo3(get)]
    pub url: String,
    #[pyo3(get)]
    pub method: Option<String>,
    #[pyo3(get)]
    pub headers: Option<BTreeMap<String, String>>,
}

#[pymethods]
impl HttpRequest {
    #[new]
    #[pyo3(signature = (url, method=None, headers=None))]
    pub fn new(
        url: String,
        method: Option<String>,
        headers: Option<BTreeMap<String, String>>,
    ) -> Self {
        HttpRequest {
            url,
            method,
            headers,
        }
    }
}

#[pyo3::pyclass(eq)]
#[derive(Debug, Clone, PartialEq)]
pub struct HttpResponse {
    pub data: Vec<u8>,
    pub status: u16,
}

#[pymethods]
impl HttpResponse {
    pub fn data<'a>(&self, py: Python<'a>) -> Bound<'a, PyBytes> {
        let bytes = PyBytes::new_bound(py, &self.data);
        bytes
    }

    pub fn status_code(&self) -> u16 {
        self.status
    }
}

#[pyo3::pyfunction]
#[pyo3(signature = (req, body=None))]
pub fn http_request(req: HttpRequest, body: Option<&[u8]>) -> PyResult<HttpResponse> {
    let req = extism_pdk::HttpRequest {
        url: req.url,
        headers: req.headers.unwrap_or_default(),
        method: req.method,
    };
    let res = extism_pdk::http::request(&req, body).map_err(error)?;
    let x = HttpResponse {
        data: res.body(),
        status: res.status_code(),
    };
    res.into_memory().free();
    Ok(x)
}

#[pyo3::pyclass(eq)]
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct MemoryHandle {
    #[pyo3(get)]
    pub offset: u64,
    #[pyo3(get)]
    pub length: u64,
}

#[pymethods]
impl MemoryHandle {
    #[new]
    pub fn new(offset: u64, length: u64) -> Self {
        MemoryHandle { offset, length }
    }
}

#[pyo3::pyfunction]
#[pyo3(name = "find")]
pub fn memory_find(offs: u64) -> PyResult<Option<MemoryHandle>> {
    if let Some(mem) = extism_pdk::Memory::find(offs) {
        Ok(Some(MemoryHandle {
            offset: mem.offset(),
            length: mem.len() as u64,
        }))
    } else {
        Ok(None)
    }
}

#[pyo3::pyfunction]
#[pyo3(name = "bytes")]
pub fn memory_bytes(py: Python<'_>, mem: MemoryHandle) -> PyResult<Bound<'_, PyBytes>> {
    let mem = extism_pdk::Memory(extism_pdk::MemoryHandle {
        offset: mem.offset,
        length: mem.length,
    });

    Ok(PyBytes::new_bound(py, &mem.to_vec()))
}

#[pyo3::pyfunction]
#[pyo3(name = "string")]
pub fn memory_string(mem: MemoryHandle) -> PyResult<String> {
    let mem = extism_pdk::Memory(extism_pdk::MemoryHandle {
        offset: mem.offset,
        length: mem.length,
    });

    mem.to_string().map_err(error)
}

#[pyo3::pyfunction]
#[pyo3(name = "free")]
pub fn memory_free(mem: MemoryHandle) -> PyResult<()> {
    let mem = extism_pdk::Memory(extism_pdk::MemoryHandle {
        offset: mem.offset,
        length: mem.length,
    });

    mem.free();
    Ok(())
}

#[pyo3::pyfunction]
#[pyo3(name = "alloc")]
pub fn memory_alloc(data: &[u8]) -> PyResult<MemoryHandle> {
    let mem = extism_pdk::Memory::from_bytes(&data).map_err(error)?;
    Ok(MemoryHandle {
        offset: mem.offset(),
        length: mem.len() as u64,
    })
}

#[pyfunction]
#[pyo3(signature = (index, *args))]
#[pyo3(name = "__invoke_host_func")]
fn invoke_host_func(index: u32, args: &Bound<'_, PyTuple>) -> PyResult<Option<MemoryHandle>> {
    let length = args.len();
    println!("INVOKE HOST INDEX: {}", index);
    let offs = unsafe {
        match length {
            0 => __invokeHostFunc_0_1(index),
            1 => __invokeHostFunc_1_1(index, args.get_item(0)?.extract::<'_, u64>()?),
            2 => __invokeHostFunc_2_1(
                index,
                args.get_item(0)?.extract::<'_, u64>()?,
                args.get_item(1)?.extract::<'_, u64>()?,
            ),
            3 => __invokeHostFunc_3_1(
                index,
                args.get_item(0)?.extract::<'_, u64>()?,
                args.get_item(1)?.extract::<'_, u64>()?,
                args.get_item(2)?.extract::<'_, u64>()?,
            ),
            4 => __invokeHostFunc_4_1(
                index,
                args.get_item(0)?.extract::<'_, u64>()?,
                args.get_item(1)?.extract::<'_, u64>()?,
                args.get_item(2)?.extract::<'_, u64>()?,
                args.get_item(3)?.extract::<'_, u64>()?,
            ),
            5 => __invokeHostFunc_5_1(
                index,
                args.get_item(0)?.extract::<'_, u64>()?,
                args.get_item(1)?.extract::<'_, u64>()?,
                args.get_item(2)?.extract::<'_, u64>()?,
                args.get_item(3)?.extract::<'_, u64>()?,
                args.get_item(4)?.extract::<'_, u64>()?,
            ),
            _ => {
                return Err(error(extism_pdk::Error::msg(
                    "Host functions with more than 5 parameters are not supported",
                )));
            }
        }
    };

    if let Some(mem) = extism_pdk::Memory::find(offs) {
        Ok(Some(MemoryHandle {
            offset: mem.offset(),
            length: mem.len() as u64,
        }))
    } else {
        Ok(None)
    }
}

#[pyfunction]
#[pyo3(signature = (index, *args))]
#[pyo3(name = "__invoke_host_func0")]
fn invoke_host_func0(index: u32, args: &Bound<'_, PyTuple>) -> PyResult<()> {
    let length = args.len();

    unsafe {
        match length {
            0 => __invokeHostFunc_0_0(index),
            1 => __invokeHostFunc_1_0(index, args.get_item(0)?.extract::<'_, u64>()?),
            2 => __invokeHostFunc_2_0(
                index,
                args.get_item(0)?.extract::<'_, u64>()?,
                args.get_item(1)?.extract::<'_, u64>()?,
            ),
            3 => __invokeHostFunc_3_0(
                index,
                args.get_item(0)?.extract::<'_, u64>()?,
                args.get_item(1)?.extract::<'_, u64>()?,
                args.get_item(2)?.extract::<'_, u64>()?,
            ),
            4 => __invokeHostFunc_4_0(
                index,
                args.get_item(0)?.extract::<'_, u64>()?,
                args.get_item(1)?.extract::<'_, u64>()?,
                args.get_item(2)?.extract::<'_, u64>()?,
                args.get_item(3)?.extract::<'_, u64>()?,
            ),
            5 => __invokeHostFunc_5_0(
                index,
                args.get_item(0)?.extract::<'_, u64>()?,
                args.get_item(1)?.extract::<'_, u64>()?,
                args.get_item(2)?.extract::<'_, u64>()?,
                args.get_item(3)?.extract::<'_, u64>()?,
                args.get_item(4)?.extract::<'_, u64>()?,
            ),
            _ => {
                return Err(error(extism_pdk::Error::msg(
                    "Host functions with more than 5 parameters are not supported",
                )));
            }
        }
    }

    Ok(())
}

#[pyo3::pymodule]
#[pyo3(name = "extism_ffi")]
pub fn make_extism_ffi_module(py: Python<'_>, module: &Bound<'_, PyModule>) -> PyResult<()> {
    let memory_module = PyModule::new_bound(py, "memory")?;
    memory_module.add_class::<MemoryHandle>()?;
    memory_module.add_function(pyo3::wrap_pyfunction!(memory_find, &memory_module)?)?;
    memory_module.add_function(pyo3::wrap_pyfunction!(memory_bytes, &memory_module)?)?;
    memory_module.add_function(pyo3::wrap_pyfunction!(memory_string, &memory_module)?)?;
    memory_module.add_function(pyo3::wrap_pyfunction!(memory_free, &memory_module)?)?;
    memory_module.add_function(pyo3::wrap_pyfunction!(memory_alloc, &memory_module)?)?;

    module.add_class::<LogLevel>()?;
    module.add_class::<HttpRequest>()?;
    module.add_class::<HttpResponse>()?;
    module.add_function(pyo3::wrap_pyfunction!(input_bytes, module)?)?;
    module.add_function(pyo3::wrap_pyfunction!(output_bytes, module)?)?;
    module.add_function(pyo3::wrap_pyfunction!(input_str, module)?)?;
    module.add_function(pyo3::wrap_pyfunction!(output_str, module)?)?;
    module.add_function(pyo3::wrap_pyfunction!(config_get, module)?)?;
    module.add_function(pyo3::wrap_pyfunction!(var_get, module)?)?;
    module.add_function(pyo3::wrap_pyfunction!(var_set, module)?)?;
    module.add_function(pyo3::wrap_pyfunction!(log, module)?)?;
    module.add_function(pyo3::wrap_pyfunction!(set_error, module)?)?;
    module.add_function(pyo3::wrap_pyfunction!(http_request, module)?)?;
    module.add_function(pyo3::wrap_pyfunction!(invoke_host_func, module)?)?;
    module.add_function(pyo3::wrap_pyfunction!(invoke_host_func0, module)?)?;
    module.add_submodule(&memory_module)?;
    Ok(())
}

#[link(wasm_import_module = "shim")]
extern "C" {
    // this import will get satisified by the import shim
    fn __invokeHostFunc_0_0(func_idx: u32);
    fn __invokeHostFunc_1_0(func_idx: u32, ptr: u64);
    fn __invokeHostFunc_2_0(func_idx: u32, ptr: u64, ptr2: u64);
    fn __invokeHostFunc_3_0(func_idx: u32, ptr: u64, ptr2: u64, ptr3: u64);
    fn __invokeHostFunc_4_0(func_idx: u32, ptr: u64, ptr2: u64, ptr3: u64, ptr4: u64);
    fn __invokeHostFunc_5_0(func_idx: u32, ptr: u64, ptr2: u64, ptr3: u64, ptr4: u64, ptr5: u64);
    fn __invokeHostFunc_0_1(func_idx: u32) -> u64;
    fn __invokeHostFunc_1_1(func_idx: u32, ptr: u64) -> u64;
    fn __invokeHostFunc_2_1(func_idx: u32, ptr: u64, ptr2: u64) -> u64;
    fn __invokeHostFunc_3_1(func_idx: u32, ptr: u64, ptr2: u64, ptr3: u64) -> u64;
    fn __invokeHostFunc_4_1(func_idx: u32, ptr: u64, ptr2: u64, ptr3: u64, ptr4: u64) -> u64;
    fn __invokeHostFunc_5_1(
        func_idx: u32,
        ptr: u64,
        ptr2: u64,
        ptr3: u64,
        ptr4: u64,
        ptr5: u64,
    ) -> u64;
}
