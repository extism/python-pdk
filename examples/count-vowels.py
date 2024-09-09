import extism
import json

@extism.plugin_fn
def count_vowels():
    input = extism.input_str()
    total = 0
    for ch in input:
        if ch in ['A', 'a', 'E', 'e', 'I', 'i', 'O', 'o', 'U', 'u']:
            total += 1
    extism.log(extism.LogLevel.Info, "Hello!")
    extism.output_json({"count": total})

