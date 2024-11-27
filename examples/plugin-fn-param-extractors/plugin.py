import extism
from dataclasses import dataclass

@dataclass
class Count(extism.Json):
    count: int

@extism.plugin_fn
def count_vowels(config: extism.Config) -> Count:
    input = extism.input_str()
    total = 0
    msg = config.get_str("message")
    extism.log(extism.LogLevel.Info, f"Input: {msg}")
    for ch in input:
        if ch in ['A', 'a', 'E', 'e', 'I', 'i', 'O', 'o', 'U', 'u']:
            total += 1
    extism.output(Count(total))

