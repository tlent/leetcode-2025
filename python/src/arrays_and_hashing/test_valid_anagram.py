from valid_anagram import is_anagram


def test_is_anagram() -> None:
    assert is_anagram("anagram", "nagaram")
    assert not is_anagram("rat", "car")
    assert is_anagram("listen", "silent")
    assert not is_anagram("a", "ab")