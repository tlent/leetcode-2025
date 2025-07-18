from typing import Optional


def length_of_longest_substring(s: str) -> int:
    seen: list[Optional[int]] = [None] * 256
    max_length = 0
    start = 0
    for i, c in enumerate(s):
        prev = seen[ord(c)]
        if prev is not None:
            start = max(start, prev + 1)
        max_length = max(max_length, i - start + 1)
        seen[ord(c)] = i
    return max_length
