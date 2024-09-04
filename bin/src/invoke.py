
def __invoke(index):
    global __all__
    return globals()[__all__[index]]()
