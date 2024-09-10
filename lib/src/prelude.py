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

IMPORT_INDEX = 0

__exports = []


def import_fn(module, name):
    global IMPORT_INDEX
    idx = IMPORT_INDEX

    def inner(func):
        def wrapper(*args):
            print(f"CALL IMPORT {idx}: {module}::{name}")
            if "return" in func.__annotations__:
                ffi.__invoke_host_func(idx, *args)
            else:
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

    def inner(*args, **kw):
        return func(*args, **kw)

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
