from n0169_majority_element import majority_element


def test_majority_element() -> None:
    assert majority_element([3, 2, 3]) == 3
    assert majority_element([2, 2, 1, 1, 1, 2, 2]) == 2
