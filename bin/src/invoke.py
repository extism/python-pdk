import traceback


def __invoke(index):
    import extism

    try:
        return extism.__exports[index]()
    except BaseException as exc:
        tb = "".join(traceback.format_tb(exc.__traceback__))
        err = f"{str(exc)}:\n{tb}"
        extism.ffi.set_error(err)
        return 1
