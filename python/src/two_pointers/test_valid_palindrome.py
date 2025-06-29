from valid_palindrome import is_palindrome, is_palindrome_two_pointer


def test_is_palindrome() -> None:
    assert is_palindrome("A man, a plan, a canal: Panama")
    assert not is_palindrome("race a car")
    assert is_palindrome(" ")


def test_is_palindrome_two_pointer() -> None:
    assert is_palindrome_two_pointer("A man, a plan, a canal: Panama")
    assert not is_palindrome_two_pointer("race a car")
    assert is_palindrome_two_pointer(" ")