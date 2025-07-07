from n0278_first_bad_version import first_bad_version


def test_first_bad_version_example_one() -> None:
    assert first_bad_version(5, lambda v: v >= 4) == 4


def test_first_bad_version_example_two() -> None:
    assert first_bad_version(1, lambda v: v >= 1) == 1
