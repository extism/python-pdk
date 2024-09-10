import traceback


def __invoke(*args):
    import extism
    index = args[0]
    a = args[1:]

    try:
        return extism.__exports[index](*a)
    except BaseException as exc:
        tb = "".join(traceback.format_tb(exc.__traceback__))
        err = f"{str(exc)}:\n{tb}"
        extism.ffi.set_error(err)
        return 1
