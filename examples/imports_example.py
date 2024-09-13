import extism

@extism.shared_fn
def do_something():
    print("Something")

@extism.shared_fn
def reflect(x: str) -> str:
    return x
