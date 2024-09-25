from typing import Union, Optional
import json
from enum import Enum
from abc import ABC, abstractmethod
from datetime import datetime
from base64 import b64encode, b64decode

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


class Codec(ABC):
    """
    Codec is used to serialize and deserialize values in Extism memory
    """

    @abstractmethod
    def encode(self) -> bytes:
        """Encode the inner value to bytes"""
        raise Exception("encode not implemented")

    @classmethod
    @abstractmethod
    def decode(s: bytes):
        """Decode a value from bytes"""
        raise Exception("encode not implemented")


class JSONEncoder(json.JSONEncoder):
    def default(self, o):
        if isinstance(o, Json):
            return o.encode()
        elif isinstance(o, bytes):
            return b64encode(o).decode()
        elif isinstance(o, datetime):
            return o.isoformat()
        return self.super().encode(o)


class JSONDecoder(json.JSONDecoder):
    def __init__(self, *args, **kwargs):
        json.JSONDecoder.__init__(self, object_hook=self.object_hook, *args, **kwargs)

    def object_hook(self, dct):
        for k, v in dct.items():
            if isinstance(v, str):
                try:
                    dct[k] = datetime.fromisoformat(v)
                    continue
                except Exception as _:
                    pass

                try:
                    dct[k] = b64decode(v.encode())
                    continue
                except Exception as _:
                    pass
            elif isinstance(v, dict):
                dct[k] = self.object_hook(v)
        return dct


class Json(Codec):
    def encode(self) -> bytes:
        v = self
        if not isinstance(self, (dict, datetime, bytes)) and hasattr(self, "__dict__"):
            if len(self.__dict__) > 0:
                v = self.__dict__
        return json.dumps(v, cls=JSONEncoder).encode()

    @classmethod
    def decode(cls, s: bytes):
        x = json.loads(s.decode(), cls=JSONDecoder)
        return cls(**x)


class JsonObject(Json, dict):
    pass


def _store(x) -> int:
    if isinstance(x, str):
        return ffi.memory.alloc(x.encode()).offset
    elif isinstance(x, bytes):
        return ffi.memory.alloc(x).offset
    elif isinstance(x, dict) or isinstance(x, list):
        return ffi.memory.alloc(json.dumps(x, cls=JSONEncoder).encode()).offset
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
        return json.loads(ffi.memory.string(mem), cls=JSONDecoder)
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


def input_json(t: Optional[type] = None):
    """Get input as JSON"""
    if t is int or t is float:
        return t(json.loads(input_str(), cls=JSONDecoder))
    if issubclass(t, Json):
        return t(**json.loads(input_str(), cls=JSONDecoder))
    return json.loads(input_str(), cls=JSONDecoder)


def output_json(x):
    """Set JSON output"""
    if isinstance(x, int) or isinstance(x, float):
        output_str(json.dumps(str(x)))
        return

    if hasattr(x, "__dict__"):
        x = x.__dict__
    output_str(json.dumps(x, cls=JSONEncoder))


def input(t: type = None):
    if t is None:
        return None
    if t is str:
        return input_str()
    elif t is bytes:
        return input_bytes()
    elif issubclass(t, Codec):
        return t.decode(input_bytes())
    elif t is dict or t is list:
        return json.loads(input_str(), cls=JSONDecoder)
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
    elif isinstance(x, Codec):
        output_bytes(x.encode())
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
        return json.loads(x, cls=JSONDecoder)

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
        return json.loads(x, cls=JSONDecoder)


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
        return json.loads(self.data_str(), cls=JSONDecoder)


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
