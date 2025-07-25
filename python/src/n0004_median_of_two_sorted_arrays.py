def find_median(a: list[int], b: list[int]) -> float:
    if len(a) > len(b):
        a, b = b, a
    m, n = len(a), len(b)
    length = m + n
    half = length // 2
    start, end = 0, m
    while True:
        i = (start + end) // 2
        j = half - i

        a_left = a[i - 1] if i > 0 else float("-inf")
        a_right = a[i] if i < m else float("inf")
        b_left = b[j - 1] if j > 0 else float("-inf")
        b_right = b[j] if j < n else float("inf")

        if a_left <= b_right and b_left <= a_right:
            if length % 2 == 0:
                return (max(a_left, b_left) + min(a_right, b_right)) / 2
            return min(a_right, b_right)
        elif a_left > b_right:
            end = i - 1
        else:
            start = i + 1
