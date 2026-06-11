from collections.abc import Mapping

from numpy.typing import NDArray
from numpy import float64

def hello_from_bin() -> str: ...

def nusselt_turbulent_smooth_duct(re: NDArray[float64], pr: NDArray[float64], f: NDArray[float64], out: NDArray[float64]): ...

def jumble(
    a: list[float],
    b: Mapping[str, str],
    c: str,
    n: int | None = None,
) -> tuple[str, list[float], dict[str, int | None]] | None: ...
