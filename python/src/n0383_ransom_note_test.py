from n0383_ransom_note import can_construct


def test_can_construct() -> None:
    assert not can_construct("a", "b")
    assert not can_construct("aa", "ab")
    assert can_construct("aa", "aab")
