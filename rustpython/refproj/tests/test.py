from math import isclose

from refproj import Cat, jumble, pet


def test_jumble_smoke() -> None:
    spam, numbers, config_maybe = jumble([1.0, 2.0, 3.0], {"x": "y"}, "eggs")

    assert spam == "6, 1, eggs, 0"
    assert len(numbers) == 3
    for actual, expected in zip(numbers, [1.1, 2.2, 3.3]):
        assert isclose(actual, expected)
    assert config_maybe == {"eggs": None}


def test_cat_json_and_pet() -> None:
    cat = Cat("Mochi")

    assert cat.to_json() == '{"name":"Mochi"}'
    assert Cat.from_json(cat.to_json()).name == "Mochi"
    assert pet(cat) == "You pet Mochi."
