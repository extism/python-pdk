import extism

@extism.shared_fn
def do_something():
    print("Something")

@extism.shared_fn
def reflect(x: str) -> str:
    return x

@extism.shared_fn
def update_dict(d: dict) -> dict:
    d["abc"] = 123
    return d
