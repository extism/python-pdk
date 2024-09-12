from typing import Union, Optional

import json
import extism_ffi as ffi

LogLevel = ffi.LogLevel
log = ffi.log
input_str = ffi.input_str
input_bytes = ffi.input_bytes
output_str = ffi.output_str
output_bytes = ffi.output_bytes

HttpRequest = ffi.HttpRequest

__exports = []

IMPORT_INDEX = 0


class Codec:
    def __init__(self, value):
        self.value = value

    def encode(self) -> bytes:
        raise Exception("encode not implemented")

    @staticmethod
    def decode(s: bytes):
        raise Exception("encode not implemented")


class Json(Codec):
    def encode(self) -> bytes:
        return json.dumps(self.value).encode()

    @staticmethod
    def decode(s: bytes):
        return Json(json.loads(s.decode()))


def _alloc(x):
    if isinstance(x, str):
        return ffi.memory.alloc(x.encode()).offset
    elif isinstance(x, bytes):
        return ffi.memory.alloc(x).offset
    elif isinstance(x, Codec):
        return ffi.memory.alloc(x.encode()).offset
    elif isinstance(x, ffi.memory.MemoryHandle):
        return a
    elif isinstance(x, int):
        return x
    else:
        raise Exception(f"Unsupported python type: {type(x)}")


def _read(t, x):
    if t == int:
        return x

    mem = ffi.memory.find(x)
    if mem is None:
        return None

    if t == str:
        return ffi.memory.string(mem)
    elif t == bytes:
        return ffi.memory.bytes(mem)
    elif t == Json:
        return Json.decode(ffi.memory.bytes(mem))
    else:
        raise Exception(f"Unsupported python type: {t}")


def import_fn(module, name):
    global IMPORT_INDEX
    idx = IMPORT_INDEX

    def inner(func):
        def wrapper(*args):
            args = [_alloc(a) for a in args]
            if "return" in func.__annotations__:
                ret = func.__annotations__["return"]
                print("RETURN", func, ret, module, name, idx, args)
                res = ffi.__invoke_host_func(idx, *args)
                return _read(ret, res)
            else:
                print("NO RETURN", func, module, name, idx, args)
                ffi.__invoke_host_func0(idx, *args)

        return wrapper
    IMPORT_INDEX += 1
    return inner


def plugin_fn(func):
    global __exports
    __exports.append(func)

    def inner():
        return func()

    return inner


def shared_fn(func):
    global __exports
    __exports.append(func)

    def inner(*args):
        args = [_alloc(a) for a in args]
        res = func(*args)
        if "return" in func.__annotations__:
            ret = func.__annotations__["return"]
            return _read(ret, res)

    return inner


def input_json():
    return json.loads(input_str())


def output_json(x):
    output_str(json.dumps(x))


class Var:
    @staticmethod
    def get_bytes(key: str) -> Optional[bytes]:
        return ffi.var_get(key)

    @staticmethod
    def get_str(key: str) -> Optional[str]:
        x = ffi.var_get(key)
        if x is None:
            return None
        return x.decode()

    @staticmethod
    def get_json(key: str):
        x = Var.get_str(key)
        if x is None:
            return x
        return json.loads(x)

    @staticmethod
    def set(key: str, value: Union[bytes, str]):
        if isinstance(value, str):
            value = value.encode()
        return ffi.var_set(key, value)


class Config:
    @staticmethod
    def get(key: str) -> Optional[str]:
        return ffi.config_get(key)

    @staticmethod
    def get_json(key: str):
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
        return self._inner.status_code()

    def data_bytes(self):
        return self._inner.data()

    def data_str(self):
        return self.data_bytes().decode()

    def data_json(self):
        return json.loads(self.data_str())


class Http:
    @staticmethod
    def request(
        url: str,
        meth: str = "GET",
        body: Optional[Union[bytes, str]] = None,
        headers: Optional[dict] = None,
    ) -> HttpResponse:
        req = HttpRequest(url, meth, headers or {})
        if body is not None and isinstance(body, str):
            body = body.encode()
        return HttpResponse(ffi.http_request(req, body))
