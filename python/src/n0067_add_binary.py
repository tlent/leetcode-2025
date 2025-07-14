def add_binary(a: str, b: str) -> str:
    result: list[str] = []
    carry = 0
    a_index = len(a) - 1
    b_index = len(b) - 1
    while a_index >= 0 or b_index >= 0 or carry > 0:
        total = carry
        if a_index >= 0:
            total += int(a[a_index])
            a_index -= 1
        if b_index >= 0:
            total += int(b[b_index])
            b_index -= 1
        carry = total // 2
        result.append(str(total % 2))
    result.reverse()
    return "".join(result)
