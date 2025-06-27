def is_palindrome(s: str) -> bool:
    clean = [c for c in s.lower() if c.isalnum()]
    return clean == clean[::-1]


def is_palindrome_two_pointer(s: str) -> bool:
    left, right = 0, len(s) - 1

    while left < right:
        while left < right and not s[left].isalnum():
            left += 1
        while left < right and not s[right].isalnum():
            right -= 1

        if s[left].lower() != s[right].lower():
            return False

        left += 1
        right -= 1

    return True


def test_is_palindrome() -> None:
    assert is_palindrome("A man, a plan, a canal: Panama")
    assert not is_palindrome("race a car")
    assert is_palindrome(" ")


def test_is_palindrome_two_pointer() -> None:
    assert is_palindrome_two_pointer("A man, a plan, a canal: Panama")
    assert not is_palindrome_two_pointer("race a car")
    assert is_palindrome_two_pointer(" ")


if __name__ == "__main__":
    test_is_palindrome()
    test_is_palindrome_two_pointer()
    print("✓ Valid Palindrome tests passed!")
