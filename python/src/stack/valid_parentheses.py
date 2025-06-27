def valid_parentheses(s: str) -> bool:
    stack: list[str] = []
    for c in s:
        if c == "(":
            stack.append(")")
        elif c == "{":
            stack.append("}")
        elif c == "[":
            stack.append("]")
        elif c != stack.pop():
            return False
    return len(stack) == 0


def test_valid_parentheses() -> None:
    assert valid_parentheses("()")
    assert valid_parentheses("()[]{}")
    assert not valid_parentheses("(]")
    assert valid_parentheses("([])")
