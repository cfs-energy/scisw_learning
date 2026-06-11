from math import isclose

from refproj import jumble


def test_jumble_smoke() -> None:
    spam, numbers, config_maybe = jumble([1.0, 2.0, 3.0], {"x": "y"}, "eggs")

    assert spam == "6, 1, eggs, 0"
    assert len(numbers) == 3
    for actual, expected in zip(numbers, [1.1, 2.2, 3.3]):
        assert isclose(actual, expected)
    assert config_maybe == {"eggs": None}
