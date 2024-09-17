from typing import Union, Optional

import json
import extism_ffi as ffi

LogLevel = ffi.LogLevel
log = ffi.log
input_str = ffi.input_str
input_bytes = ffi.input_bytes
output_str = ffi.output_str
output_bytes = ffi.output_bytes
memory = ffi.memory

HttpRequest = ffi.HttpRequest

__exports = []

IMPORT_INDEX = 0


class Codec:
    """
    Codec is used to serialize and deserialize values in Extism memory
    """

    def __init__(self, value):
        self.value = value

    def get(self):
        """Method to get the inner value"""
        return self.value

    def set(self, x):
        """Method to set in the inner value"""
        self.value = x

    def encode(self) -> bytes:
        """Encode the inner value to bytes"""
        raise Exception("encode not implemented")

    @staticmethod
    def decode(s: bytes):
        """Decode a value from bytes"""
        raise Exception("encode not implemented")


class Json(Codec):
    def encode(self) -> bytes:
        return json.dumps(self.value).encode()

    @staticmethod
    def decode(s: bytes):
        return Json(json.loads(s.decode()))


def _store(x) -> int:
    if isinstance(x, str):
        return ffi.memory.alloc(x.encode()).offset
    elif isinstance(x, bytes):
        return ffi.memory.alloc(x).offset
    elif isinstance(x, dict) or isinstance(x, list):
        return ffi.memory.alloc(json.dumps(x).encode()).offset
    elif isinstance(x, Codec):
        return ffi.memory.alloc(x.encode()).offset
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
    elif issubclass(t, Codec):
        return t.decode(ffi.memory.bytes(mem))
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


def input_json():
    """Get input as JSON"""
    return json.loads(input_str())


def output_json(x):
    """Set JSON output"""
    output_str(json.dumps(x))


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
