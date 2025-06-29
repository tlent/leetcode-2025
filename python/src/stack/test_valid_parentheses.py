from valid_parentheses import valid_parentheses


def test_valid_parentheses() -> None:
    assert valid_parentheses("()")
    assert valid_parentheses("()[]{}")
    assert not valid_parentheses("(]")
    assert valid_parentheses("([])")
