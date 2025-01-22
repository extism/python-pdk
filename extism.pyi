# WARNING THIS FILE IS AI GENERATED from
# https://github.com/extism/python-pdk/blob/main/lib/src/prelude.py and
# https://github.com/extism/python-pdk/blob/main/lib/src/py_module.rs
# It is meant purely for developer/IDE usage and should not be made available to
# extism-py
#
# Prompt used with Claude 3.5 Sonnet:
# `prelude.py` defines an `extism` module. Generate a dummy module (`.pyi`) for
# `extism` so IDEs can understand. Be sure to preserve the original comments.

"""
Extism Python Plugin Development Kit (PDK)

This module provides the interface for developing Extism plugins in Python.
It includes functionality for handling plugin I/O, HTTP requests, memory management,
configuration, and host function interactions.
"""

from typing import Any, TypeVar, Callable, Optional, Union, Dict, List, Type, TypeAlias, overload
from enum import Enum

class LogLevel(Enum):
    Trace: LogLevel
    Debug: LogLevel
    Info: LogLevel
    Warn: LogLevel
    Error: LogLevel

class MemoryHandle:
    offset: int
    length: int
    def __init__(self, offset: int, length: int) -> None: ...

class memory:
    @staticmethod
    def find(offs: int) -> Optional[MemoryHandle]: ...
    
    @staticmethod
    def bytes(mem: MemoryHandle) -> bytes: ...
    
    @staticmethod
    def string(mem: MemoryHandle) -> str: ...
    
    @staticmethod
    def free(mem: MemoryHandle) -> None: ...
    
    @staticmethod
    def alloc(data: bytes) -> MemoryHandle: ...

class HttpRequest:
    url: str
    method: Optional[str]
    headers: Optional[Dict[str, str]]
    
    def __init__(self, url: str, method: Optional[str] = None, headers: Optional[Dict[str, str]] = None) -> None: ...

class _HttpResponseInternal:
    def status_code(self) -> int: ...
    def data(self) -> bytes: ...
    headers: Dict[str, str]

class HttpResponse:
    _inner: _HttpResponseInternal
    
    def __init__(self, res: _HttpResponseInternal) -> None: ...
    
    @property
    def status_code(self) -> int:
        """Get HTTP status code"""
        ...
    
    def data_bytes(self) -> bytes:
        """Get response body bytes"""
        ...
    
    def data_str(self) -> str:
        """Get response body string"""
        ...
    
    def data_json(self) -> Any:
        """Get response body JSON"""
        ...
    
    def headers(self) -> Dict[str, str]:
        """Get HTTP response headers"""
        ...

class Http:
    @staticmethod
    def request(
        url: str,
        meth: str = "GET",
        body: Optional[Union[bytes, str]] = None,
        headers: Optional[Dict[str, str]] = None
    ) -> HttpResponse:
        """Make an HTTP request"""
        ...

T = TypeVar('T')

def log(level: LogLevel, msg: Union[str, bytes, Any]) -> None: ...

def input_bytes() -> bytes: ...
def output_bytes(result: bytes) -> None: ...
def input_str() -> str: ...
def output_str(result: str) -> None: ...

def import_fn(module: str, name: str) -> Callable[[Callable[..., Any]], Callable[..., Any]]:
    """Annotate an import function"""
    ...

def plugin_fn(func: Callable[[], Any]) -> Callable[[], Any]:
    """Annotate a function that will be called by Extism"""
    ...

def shared_fn(f: Callable[..., Any]) -> Callable[..., Any]:
    """Annotate a an export that won't be called directly by Extism"""
    ...

def input_json(t: Optional[Type[T]] = None) -> Union[T, Any]:
    """Get input as JSON"""
    ...

def output_json(x: Any) -> None:
    """Set JSON output"""
    ...

def input(t: Optional[Type[T]] = None) -> Optional[T]:
    ...

def output(x: Optional[Union[str, bytes, Dict[str, Any], List[Any], Enum]] = None) -> None:
    ...

class Var:
    @staticmethod
    def get_bytes(key: str) -> Optional[bytes]:
        """Get variable as bytes"""
        ...
    
    @staticmethod
    def get_str(key: str) -> Optional[str]:
        """Get variable as string"""
        ...
    
    @staticmethod
    def get_json(key: str) -> Optional[Any]:
        """Get variable as JSON"""
        ...
    
    @staticmethod
    def set(key: str, value: Union[bytes, str]) -> None:
        """Set a variable with a string or bytes value"""
        ...

class Config:
    @staticmethod
    def get_str(key: str) -> Optional[str]:
        """Get a config value as string"""
        ...
    
    @staticmethod
    def get_json(key: str) -> Optional[Any]:
        """Get a config value as JSON"""
        ...