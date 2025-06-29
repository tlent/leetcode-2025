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
