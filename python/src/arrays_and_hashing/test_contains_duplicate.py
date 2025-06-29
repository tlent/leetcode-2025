from contains_duplicate import has_duplicate, has_duplicate_early_return


def test_has_duplicate() -> None:
    assert has_duplicate([1, 2, 3, 1])
    assert not has_duplicate([1, 2, 3, 4])
    assert has_duplicate([1, 1, 1, 3, 3, 4, 3, 2, 4, 2])


def test_has_duplicate_early_return() -> None:
    assert has_duplicate_early_return([1, 2, 3, 1])
    assert not has_duplicate_early_return([1, 2, 3, 4])
    assert has_duplicate_early_return([1, 1, 1, 3, 3, 4, 3, 2, 4, 2])
