from typing import Union, Optional
import json
import extism_ffi as ffi

class Var:
    @staticmethod
    def get(key: str) -> bytes:
        return ffi.var_get(key)

    
    @staticmethod
    def get_str(key: str) -> str:
        return ffi.var_get(key).decode()

    @staticmethod
    def set(key: str, value: Union[bytes, str]):
        if isinstance(value, str):
            value = value.encode()
        return ffi.var_set(key, value)

class Config:
    @staticmethod
    def get(key: str) -> str:
        return ffi.config_get(key)

LogLevel = ffi.LogLevel
log = ffi.log
input_str = ffi.input_str
input_bytes = ffi.input_bytes
output_str = ffi.output_str
output_bytes = ffi.output_bytes

HttpRequest = ffi.HttpRequest
HttpResponse = ffi.HttpResponse


class Http:
    @staticmethod
    def request(url: str, meth: str = "GET", body: Optional[Union[bytes, str]] = None, headers: Optional[dict] = None) -> HttpResponse:
        req = HttpRequest(url, meth, headers or {})
        if body is not None and isinstance(body, str):
            body = body.encode()
        return ffi.http_request(req, body)

def input_json():
    return json.loads(input_str())

def output_json(x):
    output_str(json.dumps(x))
