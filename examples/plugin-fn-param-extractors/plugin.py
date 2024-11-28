import extism
from dataclasses import dataclass

@dataclass
class Count(extism.Json):
    count: int

@dataclass
class CountVowelsInput(extism.Json):
    text: str

@extism.plugin_fn
def count_vowels(cfg: extism.Config, input: str) -> Count:
    msg = cfg.get_str("message")
    extism.log(extism.LogLevel.Info, f"Config: {msg}")
    extism.log(extism.LogLevel.Info, f"Input: {input}")


@extism.plugin_fn
def count_vowels_dataclass(input: CountVowelsInput) -> Count:
    extism.log(extism.LogLevel.Info, f"Input: {input.text}")