import traceback


def __invoke(index):
    global __all__
    try:
        return globals()[__all__[index]]()
    except BaseException as exc:
        import extism

        tb = "".join(traceback.format_tb(exc.__traceback__))
        err = f"{str(exc)}:\n{tb}"
        extism.ffi.set_error(err)
        return 1
