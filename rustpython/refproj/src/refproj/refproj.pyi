from collections.abc import Mapping

from numpy.typing import NDArray
from numpy import float64

def hello_from_bin() -> str: ...

class Cat:
    name: str
    def __init__(self, name: str) -> None: ...
    def to_json(self) -> str: ...
    @staticmethod
    def from_json(json: str) -> Cat: ...

def pet(cat: Cat) -> str: ...
def nusselt_turbulent_smooth_duct(
    re: NDArray[float64],
    pr: NDArray[float64],
    f: NDArray[float64],
    out: NDArray[float64],
): ...
def jumble(
    a: list[float],
    b: Mapping[str, str],
    c: str,
    n: int | None = None,
) -> tuple[str, list[float], dict[str, int | None]] | None: ...
