def climb_stairs(n: int) -> int:
    a = 1
    b = 1
    for _ in range(2, n + 1):
        a, b = b, a + b
    return b
