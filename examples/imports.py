import extism
import json

@extism.import_fn("example", "do_something")
def do_something():
    pass

@extism.import_fn("example", "reflect")
def reflect(x: str) -> str:
    pass

@extism.plugin_fn
def count_vowels():
    input = reflect(extism.input_str())
    do_something()
    total = 0
    for ch in input:
        if ch in ['A', 'a', 'E', 'e', 'I', 'i', 'O', 'o', 'U', 'u']:
            total += 1
    extism.log(extism.LogLevel.Info, "Hello!")
    extism.output_json({"count": total})

