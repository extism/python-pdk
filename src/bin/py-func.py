import extism
import json

def count_vowels():
    input = extism.input()
    total = 0
    for ch in input:
        if ch in ['A', 'a', 'E', 'e', 'I', 'i', 'O', 'o', 'U', 'u']:
            total += 1
    extism.log(extism.LogLevel.Info, "Hello!")
    extism.output(json.dumps({"count": total}))        

