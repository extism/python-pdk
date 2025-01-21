from typing import Union, Optional
import json
from enum import Enum

import extism_ffi as ffi

LogLevel = ffi.LogLevel
input_str = ffi.input_str
input_bytes = ffi.input_bytes
output_str = ffi.output_str
output_bytes = ffi.output_bytes
memory = ffi.memory

def log(level, msg):
    if isinstance(msg, bytes):
        msg = msg.decode()
    elif not isinstance(msg, str):
        msg = str(msg)
    ffi.log(level, msg) 

HttpRequest = ffi.HttpRequest

__exports = []

IMPORT_INDEX = 0

def _store(x) -> int:
    if isinstance(x, str):
        return ffi.memory.alloc(x.encode()).offset
    elif isinstance(x, bytes):
        return ffi.memory.alloc(x).offset
    elif isinstance(x, dict) or isinstance(x, list):
        return ffi.memory.alloc(json.dumps(x).encode()).offset
    elif isinstance(x, Enum):
        return ffi.memory.alloc(str(x.value).encode()).offset
    elif isinstance(x, ffi.memory.MemoryHandle):
        return x.offset
    elif isinstance(x, int):
        return x
    elif x is None:
        return 0
    else:
        raise Exception(f"Unsupported python type: {type(x)}")


def _load(t, x):
    if t is int:
        return x

    mem = ffi.memory.find(x)
    if mem is None:
        return None

    if t is str:
        return ffi.memory.string(mem)
    elif t is bytes:
        return ffi.memory.bytes(mem)
    elif t is dict or t is list:
        return json.loads(ffi.memory.string(mem))
    elif issubclass(t, Enum):
        return t(ffi.memory.string(mem))
    elif t is ffi.memory.MemoryHandle:
        return mem
    elif t is type(None):
        return None
    else:
        raise Exception(f"Unsupported python type: {t}")


def import_fn(module, name):
    """Annotate an import function"""
    global IMPORT_INDEX
    idx = IMPORT_INDEX

    def inner(func):
        def wrapper(*args):
            args = [_store(a) for a in args]
            if "return" in func.__annotations__:
                ret = func.__annotations__["return"]
                res = ffi.__invoke_host_func(idx, *args)
                return _load(ret, res)
            else:
                ffi.__invoke_host_func0(idx, *args)

        return wrapper

    IMPORT_INDEX += 1
    return inner


def compute(func):
    def wrapper(input: str):
        # Call the host_callback
        import json
        payload = json.dumps({"name": func.__name__, "args": input})
        args = ("compute", payload)
        args = [_store(a) for a in args]
        ret = str
        # The host_callback function is imported at index 0,
        # so we make that assumption and pass index 0 to invoke the required
        # host function.
        res = ffi.__invoke_host_func(0, *args)
        return _load(ret, res)

    return wrapper


def plugin_fn(func):
    """Annotate a function that will be called by Extism"""
    global __exports
    __exports.append(func)

    def inner():
        return func()

    return inner


def shared_fn(f):
    """Annotate a an export that won't be called directly by Extism"""
    global __exports
    __exports.append(f)

    def inner(*args):
        return f(*args)

    return inner


def input_json(t: Optional[type] = None):
    """Get input as JSON"""
    if t is int or t is float:
        return t(json.loads(input_str()))
    return json.loads(input_str())


def output_json(x):
    """Set JSON output"""
    if isinstance(x, int) or isinstance(x, float):
        output_str(json.dumps(str(x)))
        return

    if hasattr(x, "__dict__"):
        x = x.__dict__
    output_str(json.dumps(x))


def input(t: type = None):
    if t is None:
        return None
    if t is str:
        return input_str()
    elif t is bytes:
        return input_bytes()
    elif t is dict or t is list:
        return json.loads(input_str())
    elif issubclass(t, Enum):
        return t(input_str())
    else:
        raise Exception(f"Unsupported type for input: {t}")


def output(x=None):
    if x is None:
        return
    if isinstance(x, str):
        output_str(x)
    elif isinstance(x, bytes):
        output_bytes(x)
    elif isinstance(x, dict) or isinstance(x, list):
        output_json(x)
    elif isinstance(x, Enum):
        output_str(x.value)
    else:
        raise Exception(f"Unsupported type for output: {type(x)}")


class Var:
    @staticmethod
    def get_bytes(key: str) -> Optional[bytes]:
        """Get variable as bytes"""
        return ffi.var_get(key)

    @staticmethod
    def get_str(key: str) -> Optional[str]:
        """Get variable as string"""
        x = ffi.var_get(key)
        if x is None:
            return None
        return x.decode()

    @staticmethod
    def get_json(key: str):
        """Get variable as JSON"""
        x = Var.get_str(key)
        if x is None:
            return x
        return json.loads(x)

    @staticmethod
    def set(key: str, value: Union[bytes, str]):
        """Set a variable with a string or bytes value"""
        if isinstance(value, str):
            value = value.encode()
        return ffi.var_set(key, value)


class Config:
    @staticmethod
    def get_str(key: str) -> Optional[str]:
        """Get a config value as string"""
        return ffi.config_get(key)

    @staticmethod
    def get_json(key: str):
        """Get a config vakye as JSON"""
        x = ffi.config_get(key)
        if x is None:
            return None
        return json.loads(x)


class HttpResponse:
    _inner: ffi.HttpResponse

    def __init__(self, res: ffi.HttpResponse):
        self._inner = res

    @property
    def status_code(self):
        """Get HTTP status code"""
        return self._inner.status_code()

    def data_bytes(self):
        """Get response body bytes"""
        return self._inner.data()

    def data_str(self):
        """Get response body string"""
        return self.data_bytes().decode()

    def data_json(self):
        """Get response body JSON"""
        return json.loads(self.data_str())

    def headers(self):
        """Get HTTP response headers"""
        return self._inner.headers or {}


class Http:
    @staticmethod
    def request(
        url: str,
        meth: str = "GET",
        body: Optional[Union[bytes, str]] = None,
        headers: Optional[dict] = None,
    ) -> HttpResponse:
        """Make an HTTP request"""
        req = HttpRequest(url, meth, headers or {})
        if body is not None and isinstance(body, str):
            body = body.encode()
        return HttpResponse(ffi.http_request(req, body))
