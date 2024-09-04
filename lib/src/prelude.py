from typing import Union
import extism_ffi as ffi

class Var:
    @staticmethod
    def get(key: str) -> bytes:
        return ffi.var_get(key)

    @staticmethod
    def set(key: str, value: Union[bytes, str]) -> bytes:
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
