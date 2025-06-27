from collections import Counter


def is_anagram(s: str, t: str) -> bool:
    return len(s) == len(t) and Counter(s) == Counter(t)


def test_is_anagram() -> None:
    assert is_anagram("anagram", "nagaram")
    assert not is_anagram("rat", "car")
    assert is_anagram("listen", "silent")
    assert not is_anagram("a", "ab")
