def has_duplicate(numbers: list[int]) -> bool:
    return len(set(numbers)) < len(numbers)


def has_duplicate_early_return(numbers: list[int]) -> bool:
    seen: set[int] = set()
    for number in numbers:
        if number in seen:
            return True
        seen.add(number)
    return False


def test_has_duplicate() -> None:
    assert has_duplicate([1, 2, 3, 1])
    assert not has_duplicate([1, 2, 3, 4])
    assert has_duplicate([1, 1, 1, 3, 3, 4, 3, 2, 4, 2])


def test_has_duplicate_early_return() -> None:
    assert has_duplicate_early_return([1, 2, 3, 1])
    assert not has_duplicate_early_return([1, 2, 3, 4])
    assert has_duplicate_early_return([1, 1, 1, 3, 3, 4, 3, 2, 4, 2])


if __name__ == "__main__":
    test_has_duplicate()
    test_has_duplicate_early_return()
    print("✓ Contains Duplicate tests passed!")
