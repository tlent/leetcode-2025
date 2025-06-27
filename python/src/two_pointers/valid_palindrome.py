def is_palindrome(s: str) -> bool:
    clean = [c for c in s.lower() if c.isalnum()]
    return clean == clean[::-1]


def is_palindrome_two_pointer(s: str) -> bool:
    start = 0
    end = len(s) - 1
    
    while start < end:
        if not s[start].isalnum():
            start += 1
        elif not s[end].isalnum():
            end -= 1
        elif s[start].lower() != s[end].lower():
            return False
        else:
            start += 1
            end -= 1
    
    return True


def test_is_palindrome() -> None:
    assert is_palindrome("A man, a plan, a canal: Panama")
    assert not is_palindrome("race a car")
    assert is_palindrome(" ")


def test_is_palindrome_two_pointer() -> None:
    assert is_palindrome_two_pointer("A man, a plan, a canal: Panama")
    assert not is_palindrome_two_pointer("race a car")
    assert is_palindrome_two_pointer(" ")
