import extism
def my_func(*args, **kwargs):
    print(extism.config_get("TEST"))
    extism.output(f"Hello, {extism.input()}!")

