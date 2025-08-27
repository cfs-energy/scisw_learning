from importlib.metadata import metadata

if __package__ is not None:
    __version__ = metadata(__package__)["Version"]


def hello() -> str:
    """
    A simple greeting
    """
    return "Hello from refproj!"
