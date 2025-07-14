from n0409_longest_palindrome import longest_palindrome


def test_longest_palindrome() -> None:
    assert longest_palindrome("abccccdd") == 7
    assert longest_palindrome("a") == 1
